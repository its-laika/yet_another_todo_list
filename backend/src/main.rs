use log::{error, info, warn};
use migration::MigratorTrait;
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod db;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = dotenvy::dotenv() {
        warn!("Could not load .env file: {e}");
    }

    let connect_options = match db::get_connect_options() {
        Ok(c) => c,
        Err(e) => {
            error!("Could not build connect options: {e}");
            return;
        }
    };

    info!("Connecting to database...");

    let connection = match db::establish_connection(connect_options).await {
        Ok(c) => c,
        Err(e) => {
            error!("Could not establish connection to database: {e}");
            return;
        }
    };

    if let Err(e) = migration::Migrator::up(&connection, None).await {
        error!("Database migration failed: {e}");
        return;
    } else {
        info!("Ran database migrations.");
    }

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

    info!("Listening on {} ...", &bind_address);

    if let Err(error) = api::init(&bind_address, api_state).await {
        error!("Server shut down with: {error}");
    };

    info!("Connection closed, server shut down.");

    if let Err(e) = connection.close().await {
        error!("Could not gracefully close database connection: {e}");
    };
}
