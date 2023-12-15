use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginDetails {
    pub email: UserEmail,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SessionId(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct UserId(pub i32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct EventId(pub i32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
    // changed_on field
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTask {
    pub user_id: UserId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: EventId,
    pub uuid: Uuid,
    pub user_id: UserId,
    pub task_id: TaskId,
    pub date_began: DateTime<Utc>,
    pub duration: i64,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewEvent {
    pub user_id: UserId,
    pub task_id: TaskId,
    pub date_began: DateTime<Utc>,
    pub duration: i64,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskWithEvents {
    pub task: Task,
    pub events: Vec<Event>,
}
