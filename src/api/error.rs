use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    UnprocessableEntity(String),
    NotFound(String),
    Conflict(String),
    Database(sqlx::Error),
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(message)
            | Self::UnprocessableEntity(message)
            | Self::NotFound(message)
            | Self::Conflict(message)
            | Self::Internal(message) => f.write_str(message),
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
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
        let status = self.status_code();

        if status.is_server_error() {
            tracing::error!(status = %status.as_u16(), error = %self, "api request failed");
        }

        (
            status,
            Json(ErrorResponse {
                error: self.client_message(),
            }),
        )
            .into_response()
    }
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Database(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn client_message(&self) -> String {
        match self {
            Self::BadRequest(message)
            | Self::UnprocessableEntity(message)
            | Self::NotFound(message)
            | Self::Conflict(message) => message.clone(),
            Self::Database(_) | Self::Internal(_) => "internal server error".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}
