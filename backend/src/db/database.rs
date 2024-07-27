use super::todo::ActiveModel;
use super::{todo, Todo};
use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    QueryFilter, Set,
};
use serde::Deserialize;
use std::env;
use std::io::{Error as IoError, ErrorKind};

type DbResult<T> = Result<T, DbErr>;

const ENV_KEY_DATABASE_URL: &str = "DATABASE_URL";

#[derive(Deserialize)]
pub struct UnsavedTodo {
    pub text: String,
    pub done: bool,
}

pub async fn establish_connection() -> Result<DatabaseConnection, IoError> {
    let database_url = env::var(ENV_KEY_DATABASE_URL).map_err(|_| {
        IoError::new(
            ErrorKind::InvalidData,
            "Missing `DATABASE_URL` in .env file",
        )
    })?;

    Database::connect(database_url).await.map_err(|_| {
        IoError::new(
            ErrorKind::ConnectionRefused,
            "Could not connect to database",
        )
    })
}

pub async fn store_todo(connection: &DatabaseConnection, todo: UnsavedTodo) -> DbResult<Todo> {
    let todo = todo::ActiveModel {
        id: Set(Uuid::new_v4()),
        text: Set(todo.text),
        done: Set(todo.done),
    };

    todo.insert(connection).await
}

pub async fn load_todos(connection: &DatabaseConnection) -> DbResult<Vec<Todo>> {
    todo::Entity::find().all(connection).await
}

pub async fn load_todo(connection: &DatabaseConnection, id: Uuid) -> DbResult<Option<Todo>> {
    todo::Entity::find()
        .filter(todo::Column::Id.eq(id))
        .one(connection)
        .await
}

pub async fn update_todo(
    connection: &DatabaseConnection,
    todo: Todo,
    update_todo: UnsavedTodo,
) -> DbResult<()> {
    let mut todo: ActiveModel = todo.into();

    todo.text = Set(update_todo.text);
    todo.done = Set(update_todo.done);

    todo.update(connection).await?;

    Ok(())
}

pub async fn delete_todo(connection: &DatabaseConnection, todo: Todo) -> DbResult<()> {
    todo.delete(connection).await?;

    Ok(())
}
