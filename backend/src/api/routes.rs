use crate::{
    api::server::ApiState,
    db::{self, Todo, UnsavedTodo},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use log::error;
use uuid::Uuid;

/// Axum endpoint handler to get all Todos (by GET-ting `/todos`)
///
/// # Returns (HTTP)
///
/// * 200, `Todo[]` - All Todos that are currently stored
/// * 500, `null`
pub async fn get_all(State(api_state): State<ApiState>) -> (StatusCode, Json<Option<Vec<Todo>>>) {
    let connection = api_state.connection.lock().await;

    let todos = match db::load_todos(&connection).await {
        Ok(t) => t,
        Err(e) => {
            error!("Could not load from database: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    (StatusCode::OK, Json(Some(todos)))
}

/// Axum endpoint handler to create a new Todo (by POST-ing `/todos`)
///
/// # Returns (HTTP)
///
/// * 201, `Todo` - The new `Todo` that has been stored
/// * 500, `null`
pub async fn create(
    State(api_state): State<ApiState>,
    Json(creation_dto): Json<UnsavedTodo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let connection = api_state.connection.lock().await;

    let unsaved_todo = UnsavedTodo {
        text: creation_dto.text,
        done: creation_dto.done,
    };

    let todo = match db::create_todo(&connection, unsaved_todo).await {
        Ok(t) => t,
        Err(e) => {
            error!("Could not store to database: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    (StatusCode::CREATED, Json(Some(todo)))
}

/// Axum endpoint handler to update an existing TODO (by PUT-ting `/todos/:id`)
///
/// # Returns (HTTP)
///
/// * 200, `Todo` - The `Todo` that has been updated
/// * 404, `null`
/// * 500, `null`
pub async fn update(
    State(api_state): State<ApiState>,
    Path(id): Path<Uuid>,
    Json(update_dto): Json<UnsavedTodo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let connection = api_state.connection.lock().await;

    let todo = match db::load_todo(&connection, &id).await {
        Ok(Some(todo)) => todo,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(None)),
        Err(e) => {
            error!("Could not load from database: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    let updated_todo = match db::update_todo(&connection, todo, update_dto).await {
        Ok(t) => t,
        Err(e) => {
            error!("Could not update database entry: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    (StatusCode::OK, Json(Some(updated_todo)))
}

/// Axum endpoint handler to delete an existing TODO (by DELETE-ing `/todos/:id`)
///
/// # Returns (HTTP)
///
/// * 200, `null`
/// * 404, `null`
/// * 500, `null`
pub async fn delete(
    State(api_state): State<ApiState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<bool>>) {
    let connection = api_state.connection.lock().await;

    let todo = match db::load_todo(&connection, &id).await {
        Ok(Some(todo)) => todo,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(None)),
        Err(e) => {
            error!("Could not load from database: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    if let Err(e) = db::delete_todo(&connection, todo).await {
        error!("Could not delete from database: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    (StatusCode::OK, Json(None))
}
