use std::net::SocketAddr;

use models::{Event, NewEvent, NewTask, Task, UserId};
use sqlx::{postgres::PgRow, PgPool, Pool, Postgres, Row};
use tracing::info;

use axum::{
    async_trait, debug_handler,
    extract::{FromRef, FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::cookie::{Cookie, Key, PrivateCookieJar, SameSite};

use crate::models::LoginDetails;
use dotenv::dotenv;

mod config;
mod models;
mod store;

// we define handler here to make the debug handler aware of the AppState
// #[debug_handler(state = AppState)]
// async fn handler(
//     State(state): State<AppState>,
//     State(store): State<store::Store>,
//     State(key): State<Key>,
// ) {
// }

#[derive(Clone)]
struct AppState {
    store: store::Store,
    key: Key,
}

#[async_trait]
impl<S> FromRequestParts<S> for store::Store
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        Ok(Self { connection: pool })
    }
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.store.connection.clone()
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for store::Store {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().ok();
    let config = config::Config::init();
    let store = store::Store::new(&config.database_url)
        .await
        .expect("Cannot connect to database");
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migrations");
    let app = router(store).await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("Listening on {addr}");
}

async fn router(store: store::Store) -> Router {
    let state = AppState {
        store,
        key: Key::generate(),
    };
    Router::new()
        .route("/", get(|| async { "Time Bandit" }))
        .route("/tasks/add_task", post(add_task))
        .route("/events/add_event", post(add_event))
        .route("/tasks", get(get_tasks_with_events))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .route("/users/register", post(register_user))
        .route("/users/login", post(login))
        .with_state(state)
}

async fn register_user(
    State(state): State<AppState>,
    Json(new_user): Json<LoginDetails>,
) -> Result<Json<String>, (StatusCode, String)> {
    let res = state
        .store
        .register_account(new_user)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

#[debug_handler(state = AppState)]
async fn login(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    Json(login): Json<LoginDetails>,
) -> Result<(PrivateCookieJar, StatusCode), StatusCode> {
    let user = match state.store.clone().get_account(login.email).await {
        Ok(user) => Ok(user),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    };
    match user {
        Ok(user) => match state.store.create_session(user, login.password).await {
            Ok(session_id) => {
                let cookie = Cookie::build(("time_bandit_auth_token_v1", session_id.0))
                    .secure(true)
                    .same_site(SameSite::Strict)
                    .http_only(true)
                    .path("/")
                    .build();
                Ok((jar.add(cookie), StatusCode::OK))
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        },
        Err(e) => Err(e),
    }
}

async fn auth(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    // we create a mutable request in order to add extensions
    mut request: Request,
    // this allows us to call the request
    next: Next,
) -> (PrivateCookieJar, Response) {
    let Some(cookie) = jar
        .get("time_bandit_auth_token_v1")
        .map(|cookie| cookie.value().to_owned())
    else {
        println!("Couldn't find a cookie in the jar");
        return (jar, (StatusCode::UNAUTHORIZED).into_response());
    };
    let find_session = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
        .bind(cookie)
        .map(|row: PgRow| UserId(row.get("user_id")))
        .fetch_one(&state.store.connection)
        .await;

    match find_session {
        Ok(res) => {
            // send the extension to the next request
            request.extensions_mut().insert(res);
            (jar, next.run(request).await)
        }
        Err(_) => (jar, (StatusCode::UNAUTHORIZED).into_response()),
    }
}

async fn add_task(
    State(state): State<AppState>,
    // this extension is is given by auth and extracted here
    Extension(user_id): Extension<UserId>,
    Json(new_task): Json<NewTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let new_task = NewTask {
        user_id,
        name: new_task.name,
        description: new_task.description,
    };
    let res = state
        .store
        .add_task(new_task)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

async fn add_event(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(new_event): Json<NewEvent>,
) -> Result<Json<Event>, (StatusCode, String)> {
    let new_event = NewEvent {
        user_id,
        task_id: new_event.task_id,
        date_began: new_event.date_began,
        duration: new_event.duration,
        notes: new_event.notes,
    };
    let res = state
        .store
        .add_event(new_event)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

#[debug_handler]
async fn get_tasks_with_events(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let res = state
        .store
        .get_tasks_by_user(user_id)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}