use axum::{Json, extract::State};
use sqlx::MySqlPool;

use crate::{models::server::Server, services::server_list::get_server_list};

pub async fn server_handler(State(pool): State<MySqlPool>) -> Result<Json<Vec<Server>>, String> {
    match get_server_list(&pool).await {
        Ok(servers) => Ok(Json(servers)),
        Err(e) => Err(e.to_string()),
    }
}
