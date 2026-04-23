use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    UnprocessableEntity(String),
    NotFound(String),
    Conflict(String),
    Database(sqlx::Error),
    Internal(String),
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound("resource not found".to_string()),
            other => Self::Database(other),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::UnprocessableEntity(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::Conflict(message) => (StatusCode::CONFLICT, message),
            Self::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {error}"),
            ),
            Self::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}
