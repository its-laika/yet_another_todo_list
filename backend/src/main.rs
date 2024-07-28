use log::{error, info};
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

mod api;
mod db;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = dotenvy::dotenv() {
        error!("Could not load .env file: {e}");
        return;
    }

    let connect_options = match db::get_connect_options() {
        Ok(c) => c,
        Err(e) => {
            error!("Could not build connect options: {e}");
            return;
        }
    };

    let connection = match db::establish_connection(connect_options).await {
        Ok(c) => c,
        Err(e) => {
            error!("Could not establish connection to database: {e}");
            return;
        }
    };

    let api_state = api::ApiState {
        connection: Arc::new(Mutex::new(connection.clone())),
    };

    let bind_address = match api::get_bind_address() {
        Ok(b) => b,
        Err(e) => {
            error!("Could not get bind address: {e}");
            return;
        }
    };

    let (_rx, tx) = oneshot::channel();

    info!("Listening on {} ...", &bind_address);

    match api::init(&bind_address, api_state, tx).await {
        Ok(()) => {
            info!("Connection closed, server shut down.");
        }
        Err(e) => {
            error!("Server shut down with: {e}");
        }
    }

    if let Err(e) = connection.close().await {
        error!("Could not gracefully close database connection: {e}");
    };
}
