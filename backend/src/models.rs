use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    prelude::Type,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginDetails {
    pub email: UserEmail,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SessionId(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Type)]
#[sqlx(transparent)]
pub struct UserId(pub i32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Type)]
#[sqlx(transparent)]
pub struct TaskEventId(pub i32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Type)]
#[sqlx(transparent)]
pub struct TaskId(pub i32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct UserEmail(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub uuid: Uuid,
    pub email: UserEmail,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub uuid: Uuid,
    pub user_id: UserId,
    pub name: String,
    pub description: String,
    pub created_on: DateTime<Utc>,
    // changed_on field
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTask {
    pub user_id: UserId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct TaskEvent {
    pub id: TaskEventId,
    pub uuid: Uuid,
    pub user_id: UserId,
    pub task_id: TaskId,
    pub date_began: DateTime<Utc>,
    pub duration: i64,
    pub notes: Option<String>,
}
impl PgHasArrayType for TaskEvent {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_event")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTaskEvent {
    pub user_id: UserId,
    pub task_id: TaskId,
    pub date_began: DateTime<Utc>,
    pub duration: i64,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskWithTaskEvents {
    pub task: Task,
    pub events: Vec<TaskEvent>,
}
