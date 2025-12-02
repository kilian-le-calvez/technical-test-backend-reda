use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: String,
    message: String,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("upstream unavailable")]
    UpstreamUnavailable,

    #[error("upstream error: {0}")]
    UpstreamError(String),

    #[error("internal server error")]
    Internal(Box<dyn std::error::Error + Send + Sync>),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ApiError::UpstreamUnavailable => StatusCode::GATEWAY_TIMEOUT,
            ApiError::UpstreamError(_) => StatusCode::BAD_GATEWAY,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            ApiError::InvalidInput(msg) =>
                ("INVALID_REQUEST", msg.clone()),

            ApiError::UpstreamUnavailable =>
                ("UPSTREAM_UNAVAILABLE".into(), "Upstream service not reachable".into()),

            ApiError::UpstreamError(_) =>
                ("UPSTREAM_ERROR".into(), "Upstream service returned error".into()),

            ApiError::Internal(_) =>
                ("INTERNAL_ERROR".into(), "Internal server error".into()),
        };

        HttpResponse::build(self.status_code()).json(
            ErrorResponse { code: code.to_string(), message }
        )
    }
}
