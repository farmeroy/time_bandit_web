use axum::{
    extract::{Path, State},
    Extension, Json,
};
use http::StatusCode;
use tracing::info;

use crate::{
    internal_error,
    models::{NewTask, Task, TaskId, TaskWithEvents, UserId},
    AppState,
};

pub async fn add_task(
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

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<TaskId>,
    Json(new_task_data): Json<NewTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let res = state
        .store
        .update_task(new_task_data, task_id)
        .await
        .map_err(internal_error)?;
    info!("{:?}", res);
    Ok(Json(res))
}

pub async fn get_user_tasks_with_events(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<Vec<TaskWithEvents>>, (StatusCode, String)> {
    let res = state
        .store
        .get_user_tasks_with_events(user_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn get_one_task_with_events(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(task_id): Path<TaskId>,
) -> Result<Json<TaskWithEvents>, (StatusCode, String)> {
    let task = state
        .store
        .get_task_with_events_by_task_id(task_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(task))
}
