use actix_web::{http::StatusCode, HttpResponse};
use log::warn;
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use sqlx::types::uuid::Error as UuidError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ApiError {
    RequestError(String),
    ServerError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    message: String,
}

impl ApiError {
    fn error_response(&self) -> String {
        match &self {
            ApiError::ServerError(_) => "Server Error".into(),
            ApiError::RequestError(e) => e.into(),
            ApiError::NotFound(e) => e.into(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for ApiError {
    fn from(err: actix_web::error::Error) -> Self {
        ApiError::ServerError(err.to_string())
    }
}

impl From<SqlxError> for ApiError {
    fn from(err: SqlxError) -> Self {
        ApiError::ServerError(err.to_string())
    }
}

impl From<UuidError> for ApiError {
    fn from(_err: UuidError) -> Self {
        ApiError::RequestError("Invalid identifier".to_string())
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match &self {
            ApiError::ServerError(err) => {
                warn!("ServerError: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::RequestError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiErrorResponse {
            message: self.error_response(),
        })
    }
}
