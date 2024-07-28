//! Module for all database and model handling.
//!
//! Contains functions to connect to a database, models
//! and all necessary functions to create, read, update
//! and delete those models from the database.

mod database;
pub mod todo;

pub use database::create_todo;
pub use database::delete_todo;
pub use database::establish_connection;
pub use database::get_connect_options;
pub use database::load_open_todos;
pub use database::load_todo;
pub use database::update_todo;
pub use database::UnsavedTodo;

pub use todo::Model as Todo;
