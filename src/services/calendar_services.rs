use chrono::{NaiveDate, Utc};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use sqlx::{MySqlPool, pool};

use crate::models::calendar::CalendarData;
use crate::models::calendar::CalendarRequest;

#[derive(Debug)]
pub struct AppError(pub String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError(format!("HTTP error: {}", err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError(format!("Database error: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError(format!("JSON error: {}", err))
    }
}

pub async fn fetch_jx3_data(request: &CalendarRequest) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let url = "https://www.jx3api.com/data/active/calendar";
    let response = client
        .post(url)
        .json(request)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

pub async fn get_today_calendar(
    pool: &MySqlPool,
    server: String,
) -> Result<CalendarData, AppError> {
    let today = Utc::now().date_naive();
    let start_of_day = today.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = today.and_hms_opt(23, 59, 59).unwrap();

    if let Some(existing) = sqlx::query_as!(
        CalendarData,
        r#"SELECT id, server_name, json_data, create_time 
           FROM calendar 
           WHERE server_name = ? AND create_time BETWEEN ? AND ?"#,
        server,
        start_of_day,
        end_of_day
    )
    .fetch_optional(pool)
    .await?
    {
        return Ok(existing);
    }

    let request = CalendarRequest {
        server,
        num: Some(0),
    };

    let api_response = fetch_jx3_data(&request).await?;

    let now = Utc::now().naive_utc();

    sqlx::query!(
        r#"INSERT IGNORE INTO calendar (server_name, json_data, create_time) 
       VALUES (?, ?, ?)"#,
        request.server,
        api_response,
        now
    )
    .execute(pool)
    .await?;

    // 获取刚插入的 ID
    let id: i64 = sqlx::query_scalar!("SELECT LAST_INSERT_ID()")
        .fetch_one(pool)
        .await?
        .try_into()
        .unwrap();

    // 5. 构造返回对象
    let result = CalendarData {
        id,
        server_name: request.server,
        json_data: api_response,
        create_time: now,
    };

    Ok(result)
}
