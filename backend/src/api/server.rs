use std::{
    env::{self, VarError},
    io::Error,
    sync::Arc,
};

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use log::info;
use sea_orm::DatabaseConnection;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{oneshot, Mutex},
};

use super::routes;

const ENV_KEY_BIND_ADDRESS: &str = "BIND_ADDRESS";

#[derive(Clone)]
pub struct ApiState {
    pub connection: Arc<Mutex<DatabaseConnection>>,
}

pub async fn init<A: ToSocketAddrs + Send>(
    address: A,
    api_state: ApiState,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/todos", get(routes::get_all))
        .route("/todos", post(routes::create_new))
        .route("/todos/:id", put(routes::update))
        .route("/todos/:id", delete(routes::delete))
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

pub fn get_bind_address() -> Result<String, VarError> {
    env::var(ENV_KEY_BIND_ADDRESS)
}
