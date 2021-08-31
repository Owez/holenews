//! Contains [Result], [Error] and implementations

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

/// Type cover for a result based on the [Error] variants
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-specific central error variants
#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    LocationNotFound,
    WarNotFound(i64),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(_) => write!(f, "Database error"),
            Error::LocationNotFound => write!(f, "Map location provided could not be found"),
            Error::WarNotFound(num) => write!(f, "War number {} could not be found", num),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::LocationNotFound | Error::WarNotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: format!("{}", self),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
