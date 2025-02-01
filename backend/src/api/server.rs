use super::routes;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use log::info;
use sea_orm::DatabaseConnection;
use std::{
    env::{self, VarError},
    io::Error,
    sync::Arc,
};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{oneshot, Mutex},
};

/// Global state for all API handlers.
/// Initialize with an existing database connection.
///
/// # Examples
///
/// ```rust
/// let connection = sea_orm::Database::connect("...").await.unwrap();
/// let api_state = api::ApiState {
///     connection: std::sync::Arc::new(tokio::sync::Mutex::new(connection.clone())),
/// };
/// ```
#[derive(Clone)]
pub struct ApiState {
    pub connection: Arc<Mutex<DatabaseConnection>>,
}

/// Initiates the API server
///
/// # Arguments
///
/// * `address` - The (socket) address to bind the HTTP server on
/// * `api_state` - The state object that will be passed into the handlers.
///   Contains a database connection.
/// * `shutdown_rx` - Oneshot receiver that initiates a graceful shutdown
///   of the HTTP server.
///
/// # Examples
///
/// ```rust
/// let (_rx, tx) = tokio::sync::oneshot::channel();
/// let address = "127.0.0.1:8080";
/// let connection = sea_orm::Database::connect("...").await.unwrap();
/// let api_state = api::ApiState {
///     connection: std::sync::Arc::new(tokio::sync::Mutex::new(connection.clone())),
/// };
///
/// if api::init(&bind_address, api_state, tx).await.is_ok() {
///     println!("API shut down.");
/// } else {
///     println!("API shut down with error!");
/// }
/// ```
pub async fn init<A: ToSocketAddrs + Send>(
    address: A,
    api_state: ApiState,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/todos/open", get(routes::get_open))
        .route("/todos", post(routes::create))
        .route("/todos/{id}", put(routes::update))
        .route("/todos/{id}", delete(routes::delete))
        .with_state(api_state);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();

            info!("API received shutdown signal...");
        })
        .await?;

    Ok(())
}

/// Returns the address on which the API server should bind on.
///
/// Bind address is defined by the environment variable
/// `BIND_ADDRESS`. Set either by _.env_ file or `export BIND_ADDRESS=...`.
///
/// # Examples
///
/// ```rust
/// let bind_address = api::get_bind_address().unwrap();
/// println!("Will listen on {bind_address}...");
/// ```
pub fn get_bind_address() -> Result<String, VarError> {
    env::var(ENV_KEY_BIND_ADDRESS)
}

/// Environment key for the address on which the API server
/// should bind on.
/// See [get_bind_address](get_bind_address).
const ENV_KEY_BIND_ADDRESS: &str = "BIND_ADDRESS";
