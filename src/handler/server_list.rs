use axum::{Json, extract::State};
use reqwest::StatusCode;
use sqlx::MySqlPool;

use crate::{
    core::error::AppError,
    models::{response::ApiResponse, server::Server},
    services::server_list::get_server_list,
};

pub async fn server_handler(
    State(pool): State<MySqlPool>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<Server>>>), AppError> {
    let server_list = get_server_list(&pool).await?;
    Ok((StatusCode::OK, Json(ApiResponse::success(server_list))))
}
