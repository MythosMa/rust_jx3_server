use axum::{Router, routing::get};
use sqlx::MySqlPool;

use crate::handler::calendar::calendar_handler;

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/calendar", get(calendar_handler))
}
