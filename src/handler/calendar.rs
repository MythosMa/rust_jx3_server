use axum::{
    Json,
    extract::{Query, State},
};
use sqlx::MySqlPool;

use crate::{
    core::error::AppError,
    models::{
        calendar::{CalendarData, CalendarRequest},
        response::ApiResponse,
    },
    services::calendar::get_today_calendar,
};

pub async fn calendar_handler(
    State(pool): State<MySqlPool>,
    Query(params): Query<CalendarRequest>,
) -> Result<Json<ApiResponse<CalendarData>>, AppError> {
    let calendar_data = get_today_calendar(&pool, params.server).await?;
    Ok(Json(ApiResponse::success(calendar_data)))
}
