use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Error, Row,
};

use crate::{
    models::{Event, EventId, NewEvent, NewTask, Task, TaskId, TaskWithEvents, User, UserId},
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

    pub async fn add_event(self, new_event: NewEvent) -> Result<Event, Error> {
        match sqlx::query(
            "INSERT INTO events (user_id, task_id, date_began, duration, notes)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, uuid, user_id, task_id, date_began, duration, notes",
        )
        .bind(new_event.user_id.0)
        .bind(new_event.task_id.0)
        .bind(new_event.date_began)
        .bind(new_event.duration)
        .bind(new_event.notes)
        .map(|row: PgRow| Event {
            id: EventId(row.get("id")),
            uuid: row.get("uuid"),
            user_id: UserId(row.get("user_id")),
            task_id: TaskId(row.get("task_id")),
            date_began: row.get("date_began"),
            duration: row.get("duration"),
            notes: Some(row.get("notes")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(event) => Ok(event),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_tasks_by_user(self, user_id: UserId) -> Result<Vec<Task>, Error> {
        match sqlx::query(
            "
            SELECT id, uuid, user_id, name, description, created_on
            FROM tasks
            WHERE user_id = $1
            ",
        )
        .bind(user_id.0)
        .map(|row: PgRow| Task {
            id: TaskId(row.get("id")),
            uuid: row.get("uuid"),
            user_id: UserId(row.get("user_id")),
            name: row.get("name"),
            description: Some(row.get("description")),
            created_on: row.get("created_on"),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(tasks) => Ok(tasks),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(err)
            }
        }
    }

    pub async fn get_events_by_task(self, task_id: TaskId) -> Result<Vec<Event>, Error> {
        match sqlx::query(
            "
            SELECT id, uuid, task_id, user_id, date_began, duration, notes
            FROM events
            WHERE task_id = $1
            ",
        )
        .map(|row: PgRow| Event {
            id: EventId(row.get("id")),
            uuid: row.get("uuid"),
            task_id: TaskId(row.get("task_id")),
            user_id: UserId(row.get("user_id")),
            date_began: row.get("date_began"),
            duration: row.get("duration"),
            notes: Some(row.get("notes")),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(events) => Ok(events),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(err)
            }
        }
    }
}
