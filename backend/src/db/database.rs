use super::todo::ActiveModel;
use super::{todo, Todo};
use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr,
    EntityTrait, ModelTrait, QueryFilter, Set,
};
use serde::Deserialize;
use std::env::{self, VarError};

/// Representation of a [Todo](../todo.rs#Model) that should be saved to
/// database.
///
/// Will be used to create a new Todo or update an existing one. It equals
/// the [Todo](../todo.rs#Model) but does not contain an Id.
///
/// # Examples
///
/// ```rust
/// let unsaved_todo = db::UnsavedTodo {
///     text: String::from("Be gay, do crime!"),
///     done: false
/// };
///
/// // store / update Todo...
/// ```
#[derive(Deserialize)]
pub struct UnsavedTodo {
    /// Content of the Todo
    pub text: String,
    /// Indicator whether the Todo has been completed
    pub done: bool,
}

/// Establishes a connection to database
///
/// # Arguments
///
/// * `connect_options` - E.g. a database connection string
///
/// # Returns
///
/// An active connection or an `sea_orm::DbErr` if connection fails.
///
/// # Examples
///
/// ```rust
/// let connect_options = "postgres://user:password@host:port/database";
/// let connection = db::establish_connection(connect_options).unwrap();
///
/// // do stuff, fire some SQLs...
/// ```
pub async fn establish_connection<A: Into<ConnectOptions>>(
    connect_options: A,
) -> Result<DatabaseConnection, DbErr> {
    Database::connect(connect_options).await
}

/// Get connect options by environment
///
/// Tries building `sea_orm::ConnectOptions` by connection string
/// that is defined by an environment variable _DATABASE\_URL_.
/// Returns `std::env::VarError` if variable cannot be loaded.
///
/// # Examples
///
/// ```rust
/// let connect_options = db::get_connect_options().unwrap();
/// // do stuff, create a connection...
/// ```
pub fn get_connect_options() -> Result<ConnectOptions, VarError> {
    let connection_string = env::var(ENV_KEY_DATABASE_URL)?;
    Ok(ConnectOptions::from(connection_string))
}

/// Stores a new dto and returns it.
///
/// Creates, stores a Todo by given `creation_dto` and returns it.
///
/// # Arguments
///
/// * `connection` - A connection to the database where the Todo
///   will be stored
/// * `creation_dto` - The DTO that contains the necessary information
///   of the Todo
///
/// # Returns
///
/// The new, stored Todo.
///
/// # Examples
///
/// * see [Todo creation route](../api/routes.rs#create)
pub async fn create_todo(
    connection: &DatabaseConnection,
    creation_dto: UnsavedTodo,
) -> DbResult<Todo> {
    let todo = todo::ActiveModel {
        id: Set(Uuid::new_v4()), // no auto generation, so do it manually
        text: Set(creation_dto.text),
        done: Set(creation_dto.done),
    };

    todo.insert(connection).await
}

/// Loads *open* Todos out of the database
///
/// # Arguments
///
/// * `connection` - A connection to the database where the Todos
///   are stored
///
/// # Examples
///
/// * see [Todo loading route](../api/routes.rs#get_all)
pub async fn load_open_todos(connection: &DatabaseConnection) -> DbResult<Vec<Todo>> {
    todo::Entity::find()
        .filter(todo::Column::Done.eq(false))
        .all(connection)
        .await
}

/// Loads a Todo out of the database by given `id`
///
/// # Arguments
///
/// * `connection` - A connection to the database where the Todo
///   is stored
/// * `id` - Uuid of the Todo
///
/// # Examples
///
/// * see [Todo update route](../api/routes.rs#update)
/// * see [Todo deletion route](../api/routes.rs#delete)
pub async fn load_todo(connection: &DatabaseConnection, id: &Uuid) -> DbResult<Option<Todo>> {
    todo::Entity::find()
        .filter(todo::Column::Id.eq(*id))
        .one(connection)
        .await
}

/// Updates a Todo in the database
///
/// # Arguments
///
/// * `connection` - A connection to the database where the Todo
///   is stored
/// * `todo` - The Todo to update
/// * `update_dto` - A DTO containing the updated values that
///   should be persisted
///
/// # Examples
///
/// * see [Todo update route](../api/routes.rs#update)
///
/// ```rust
/// let connection: sea_orm::DatabaseConnection = ...;
///
/// let todo = db::load_todo(&connection, &id).await.unwrap();
/// let update_dto = db::UnsavedTodo {
///     text: String::from("Be gay, do crime!"),
///     done: true
/// };
///
/// let new_todo = db::update_todo(&connection, todo, update_dto).await.unwrap();
///
/// assert_eq!(new_todo.text, "Be gay, do crime!");
/// assert!(new_todo.done);
/// ```
pub async fn update_todo(
    connection: &DatabaseConnection,
    todo: Todo,
    update_dto: UnsavedTodo,
) -> DbResult<Todo> {
    let mut todo: ActiveModel = todo.into();

    todo.text = Set(update_dto.text);
    todo.done = Set(update_dto.done);

    todo.update(connection).await
}

/// Deletes a Todo of the database by given `id`
///
/// # Arguments
///
/// * `connection` - A connection to the database where the Todo
///   is stored
/// * `id` - Uuid of the Todo
///
/// # Examples
///
/// * see [Todo deletion route](../api/routes.rs#delete)
pub async fn delete_todo(connection: &DatabaseConnection, todo: Todo) -> DbResult<()> {
    todo.delete(connection).await?;
    Ok(())
}

/// `Result` type for functions that work with the database and
/// may return a `sea_orm::DbErr`
type DbResult<T> = Result<T, DbErr>;

/// Environment key for the connection string of the database.
/// See [get_connect_options](get_connect_options).
const ENV_KEY_DATABASE_URL: &str = "DATABASE_URL";
