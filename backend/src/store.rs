use bcrypt;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Error, Row,
};
use tracing::info;

use crate::{
    models::{Event, EventId, NewEvent, NewTask, SessionId, Task, TaskId, User, UserEmail, UserId},
    LoginDetails,
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

    pub async fn register_account(self, new_user: LoginDetails) -> Result<String, Error> {
        let hashed_password = bcrypt::hash(new_user.password, 10).unwrap();
        let new_user = LoginDetails {
            email: new_user.email,
            password: hashed_password,
        };
        match sqlx::query(
            "INSERT INTO users (email, password)
            VALUES ($1, $2)",
        )
        .bind(new_user.email.0)
        .bind(new_user.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok("Created account!".to_string()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_account(self, email: UserEmail) -> Result<User, Error> {
        match sqlx::query("SELECT id, uuid, email, password FROM users WHERE email = $1")
            .bind(email.0)
            .map(|row: PgRow| User {
                id: UserId(row.get("id")),
                uuid: row.get("uuid"),
                email: UserEmail(row.get("email")),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                info!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn create_session(self, user: User, password: String) -> Result<SessionId, Error> {
        info!("Create session");
        if bcrypt::verify(password, &user.password).unwrap_or_default() == false {
            println!("Unauthorized");
            return Err(Error::RowNotFound);
        };

        let session_id = rand::random::<u64>().to_string();

        match sqlx::query(
            "
            INSERT INTO sessions (session_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET session_id = EXCLUDED.session_id
            RETURNING session_id
            ",
        )
        .bind(&session_id)
        .bind(user.id.0)
        .map(|row: PgRow| SessionId(row.get("session_id")))
        .fetch_one(&self.connection)
        .await
        {
            Ok(session_id) => Ok(session_id),
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

    pub async fn update_task(self, new_task: NewTask, task_id: TaskId) -> Result<Task, Error> {
        match sqlx::query(
            "UPDATE tasks SET name = $1, description = $2 WHERE id = $3 AND user_id = $4 RETURNING 
             id, uuid, user_id, name, description, created_on",
        )
        .bind(new_task.name)
        .bind(new_task.description)
        .bind(task_id.0)
        .bind(new_task.user_id.0)
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
        .bind(task_id.0)
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
            Ok(events) => {
                info!("Events from DB: {:?}", events);
                Ok(events)
            }
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(err)
            }
        }
    }

    pub async fn get_task_by_id(self, task_id: TaskId) -> Result<Task, Error> {
        match sqlx::query(
            "
            SELECT id, uuid, user_id, name, description, created_on
            FROM tasks
            WHERE id = $1
            ",
        )
        .bind(task_id.0)
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
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(err)
            }
        }
    }

    // pub async fn get_task_with_events_by_task_id(
    //     self,
    //     task_id: TaskId,
    // ) -> Result<Vev<TaskWithEvents>, Error> {
    //     let task = self
    //         .clone()
    //         .get_task_by_id(task_id.clone())
    //         .await
    //         .unwrap_err();
    //     let events = self.get_events_by_task(task_id.clone()).await.unwrap_err();
    // }
}
