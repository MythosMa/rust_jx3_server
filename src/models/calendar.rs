use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CalendarData {
    pub id: i64,
    pub server_name: String,
    pub json_data: JsonValue,
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarRequest {
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<u8>,
}
