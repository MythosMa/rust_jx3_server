use crate::{core::error::AppError, models::server::Server};
use sqlx::MySqlPool;

pub async fn get_server_list(pool: &MySqlPool) -> Result<Vec<Server>, AppError> {
    let servers = sqlx::query_as::<_, Server>("SELECT * FROM server_list")
        .fetch_all(pool)
        .await?;
    Ok(servers)
}
