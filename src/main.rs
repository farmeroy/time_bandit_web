use std::net::SocketAddr;

use models::{Event, NewEvent, NewTask, Task, User, UserEmail, UserId};
use tower_http::trace::TraceLayer;
use tracing::info;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::models::NewUser;
use dotenv::dotenv;

mod config;
mod models;
mod store;

#[derive(Clone)]
struct AppState {
    store: store::Store,
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
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    info!("Listening on {addr}");
}

async fn router(store: store::Store) -> Router {
    let state = AppState { store };
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(|| async { "Time Bandit" }))
        .route("/users/add_user", post(add_user))
        .route("/tasks/add_task", post(add_task))
        .route("/events/add_event", post(add_event))
        .route("/tasks/:user_id", get(get_tasks_with_events))
        .with_state(state)
}

async fn add_user(
    State(state): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<bool>, (StatusCode, String)> {
    let res = state
        .store
        .add_user(new_user)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

async fn add_task(
    State(state): State<AppState>,
    Json(new_task): Json<NewTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
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
    Json(new_event): Json<NewEvent>,
) -> Result<Json<Event>, (StatusCode, String)> {
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
    Path(user_id): Path<UserId>,
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
