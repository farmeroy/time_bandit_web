use axum::{extract::State, Extension, Json};
use http::StatusCode;
use tracing::info;

use crate::{
    internal_error,
    models::{Event, NewEvent, UserId},
    AppState,
};

pub async fn add_event(
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
