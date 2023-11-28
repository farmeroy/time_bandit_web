use std::net::SocketAddr;

use models::{Event, NewEvent, NewTask, Task, User, UserEmail};
use tower_http::trace::TraceLayer;
use tracing::info;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::models::NewUser;

mod models;
mod store;

#[derive(Clone)]
struct AppState {
    store: store::Store,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let app = router().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    info!("Listening on {addr}");
}

async fn router() -> Router {
    let store = store::Store::new("postgres://raffaele:rust_crud@localhost:5432/time_bandit")
        .await
        .expect("Cannot connect to database");
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migrations");
    let state = AppState { store };
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(|| async { "Time Bandit" }))
        .route("/users/add_user", post(add_user))
        .route("/tasks/add_task", post(add_task))
        .route("/events/add_event", post(add_event))
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

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
