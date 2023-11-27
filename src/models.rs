use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: UserEmail,
}

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
    pub uuid: String,
    pub email: UserEmail,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub uuid: Uuid,
    pub user_id: UserId,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct NewTask {
    pub user_id: UserId,
    pub name: String,
    pub description: Option<String>,
}

pub struct Event {
    pub id: EventId,
    pub uuid: String,
    pub user_id: UserId,
    pub task_id: TaskId,
    pub date_began: DateTime<Utc>,
    pub duration: i32,
    pub notes: String,
}