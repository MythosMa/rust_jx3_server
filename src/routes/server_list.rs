use axum::{Router, routing::get};
use sqlx::MySqlPool;

use crate::handler::server_list::server_handler;

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/server-list", get(server_handler))
}
