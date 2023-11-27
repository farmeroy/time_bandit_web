use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Error, Row,
};

use crate::{
    models::{NewTask, Task, TaskId, UserId},
    NewUser,
};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish a DB connection: {}", e),
        };
        Ok(Store {
            connection: db_pool,
        })
    }

    pub async fn add_user(self, new_user: NewUser) -> Result<bool, Error> {
        match sqlx::query(
            "INSERT INTO users (email)
            VALUES ($1)",
        )
        .bind(new_user.email.0)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub async fn add_task(self, new_task: NewTask) -> Result<Task, Error> {
        match sqlx::query(
            // @todo: authenticate and authorize the user id from a session?
            "INSERT INTO tasks (user_id, name, description)
            Values ($1, $2, $3)
            RETURNING id, uuid, user_id, name, description, created_on",
        )
        .bind(new_task.user_id.0)
        .bind(new_task.name)
        .bind(new_task.description)
        .map(|row: PgRow| Task {
            id: TaskId(row.get("id")),
            uuid: row.get("uuid"),
            user_id: UserId(row.get("user_id")),
            name: row.get("name"),
            description: Some(row.get("description")),
            created_on: row.get("created_on"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(task) => Ok(task),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}
