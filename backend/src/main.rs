use std::net::SocketAddr;

use http::{
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION,
        CONTENT_TYPE, COOKIE, ORIGIN,
    },
    HeaderValue,
};
use routes::{
    auth::{auth_middleware, get_session},
    events::add_event,
    tasks::{add_task, get_one_task_with_events, get_user_tasks_with_events, update_task},
};
use sqlx::{PgPool, Pool, Postgres};
use tower_http::cors::CorsLayer;
use tracing::info;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, State},
    http::{request::Parts, Method, StatusCode},
    middleware::{self},
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;

use crate::models::LoginDetails;
use crate::routes::users::{login, register_user};
use dotenv::dotenv;

mod config;
mod models;
mod routes;
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
        .route("/tasks", post(add_task))
        .route("/tasks", get(get_user_tasks_with_events))
        .route(
            "/tasks/:task_id",
            get(get_one_task_with_events).put(update_task),
        )
        .route("/events/add_event", post(add_event))
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

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
