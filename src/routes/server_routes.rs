use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use sqlx::MySqlPool;

use crate::{models::server::Server, services::server_services::get_server_list};

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/server_list", get(server_handler))
}
async fn server_handler(State(pool): State<MySqlPool>) -> Result<Json<Vec<Server>>, String> {
    match get_server_list(&pool).await {
        Ok(servers) => Ok(Json(servers)),
        Err(e) => Err(e.to_string()),
    }
}
