//! Module for all HTTP handling
//!
//! Contains functions to set up a HTTP server that
//! provides the Todo API.

mod routes;
mod server;

#[allow(clippy::module_name_repetitions)]
pub use server::ApiState;
pub use server::{get_bind_address, init};
