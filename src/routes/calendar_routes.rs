use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use sqlx::MySqlPool;

use crate::{models::calendar::CalendarRequest, services::calendar_services::get_today_calendar};

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/calendar", get(calendar_handler))
}

pub async fn calendar_handler(
    State(pool): State<MySqlPool>,
    Query(params): Query<CalendarRequest>,
) -> Result<Json<crate::models::calendar::CalendarData>, String> {
    match get_today_calendar(&pool, params.server).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e.to_string()),
    }
}
