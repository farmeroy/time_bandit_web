use std::net::SocketAddr;

use http::{
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION,
        CONTENT_TYPE, COOKIE, ORIGIN,
    },
    HeaderValue,
};
use models::{Event, NewEvent, NewTask, Task, TaskId, TaskWithEvents, UserId};
use sqlx::{postgres::PgRow, PgPool, Pool, Postgres, Row};
use tower_http::cors::CorsLayer;
use tracing::info;

use axum::{
    async_trait, debug_handler,
    extract::{FromRef, FromRequestParts, Path, Request, State},
    http::{request::Parts, Method, StatusCode},
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
    let cors = CorsLayer::new()
        .allow_headers([
            COOKIE,
            ORIGIN,
            AUTHORIZATION,
            ACCEPT,
            ACCESS_CONTROL_ALLOW_ORIGIN,
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            CONTENT_TYPE,
        ])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());
    Router::new()
        .route("/tasks/add_task", post(add_task))
        .route("/events/add_event", post(add_event))
        .route("/tasks", get(get_user_tasks))
        .route("/tasks/:task_id", get(get_task_with_events))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .route("/auth", get(get_session))
        .route("/users/register", post(register_user))
        .route("/users/login", post(login))
        .route("/", get(|| async { "Time Bandit" }))
        .with_state(state)
        .layer(cors)
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

/// A simple endpoint to check if the cookie session is valid
/// This is used in a <Session/> wrapper in the frontend
async fn get_session(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<Json<UserId>, (StatusCode, String)> {
    let Some(cookie) = jar
        .get("time_bandit_auth_token_v1")
        .map(|cookie| cookie.value().to_owned())
    else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    };
    let res = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
        .bind(cookie)
        .map(|row: PgRow| UserId(row.get("user_id")))
        .fetch_one(&state.store.connection)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

async fn auth_middleware(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    // we create a mutable request in order to add extensions
    mut request: Request,
    // this allows us to call the request
    next: Next,
) -> Result<(PrivateCookieJar, Response), Response> {
    let Some(cookie) = jar
        .get("time_bandit_auth_token_v1")
        .map(|cookie| cookie.value().to_owned())
    else {
        let res = (StatusCode::UNAUTHORIZED).into_response();
        info!("{:?}", res);
        return Err(res);
    };
    let find_session = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
        .bind(cookie)
        .map(|row: PgRow| UserId(row.get("user_id")))
        .fetch_one(&state.store.connection)
        .await;

    match find_session {
        Ok(res) => {
            // send the extension to the next request
            info!("{:?}", res);
            request.extensions_mut().insert(res);
            Ok((jar, next.run(request).await))
        }
        Err(_) => {
            let res = StatusCode::UNAUTHORIZED.into_response();
            info!("{:?}", res);
            return Err(res);
        }
    }
}

async fn add_task(
    State(state): State<AppState>,
    // this extension is given by auth and extracted here
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
async fn get_events_by_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(task_id): Path<TaskId>,
) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    info!("Request by user: {:?}", user_id);
    let res = state
        .store
        .get_events_by_task(task_id)
        .await
        .map_err(internal_error)?;
    info!("Request Events: {:?}", res);
    Ok(Json(res))
}

#[debug_handler]
async fn get_user_tasks(
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

async fn get_task_with_events(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(task_id): Path<TaskId>,
) -> Result<Json<TaskWithEvents>, (StatusCode, String)> {
    let task = state
        .store
        .clone()
        .get_task_by_id(task_id.clone())
        .await
        .map_err(internal_error)?;
    let events = state
        .store
        .clone()
        .get_events_by_task(task_id.clone())
        .await
        .map_err(internal_error)?;
    Ok(Json(TaskWithEvents { task, events }))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
