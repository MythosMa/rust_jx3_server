// src/core/error.rs

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest;
use serde_json;
use sqlx;
use thiserror::Error;

use crate::models::response::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found")]
    NotFound,

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("JSON processing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Internal server error")]
    InternalError,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, msg) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, 404, self.to_string()),
            AppError::InvalidParams(_) => (StatusCode::BAD_REQUEST, 400, self.to_string()),
            AppError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 500, self.to_string())
            }
            AppError::ExternalApiError(_) => (StatusCode::BAD_GATEWAY, 502, self.to_string()),
            AppError::HttpRequestError(_) => (StatusCode::BAD_GATEWAY, 502, self.to_string()),
            AppError::JsonError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500, self.to_string()),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                500,
                "Internal server error".into(),
            ),
        };

        let resp = ApiResponse::<()> {
            code,
            msg,
            data: None,
        };
        (status, Json(resp)).into_response()
    }
}
