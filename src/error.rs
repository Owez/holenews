//! Contains [Result], [Error] and implementations

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::{fmt, io};

/// Type cover for a result based on the [Error] variants
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-specific central error variants
#[derive(Debug)]
pub enum Error {
    StaticOpen(io::Error),
    Database(sqlx::Error),
    LocationNotFound,
    WarNotFound(i64),
    BattleNotFound(i64),
    TemplateRender(tera::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Self::TemplateRender(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: add mixed logging here too
        match self {
            Error::StaticOpen(_) => write!(f, "Could not retrieve html file from static files"),
            Error::Database(_) => write!(f, "Database error"),
            Error::LocationNotFound => write!(f, "Map location provided could not be found"),
            Error::WarNotFound(num) => write!(f, "War number {} could not be found", num),
            Error::BattleNotFound(id) => write!(f, "Battle id {} could not be found", id),
            Error::TemplateRender(_) => write!(f, "Could not properly render html template"),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        // TODO: make this better and add tera templating?
        match self {
            Error::StaticOpen(_) | Error::Database(_) | Error::TemplateRender(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::LocationNotFound | Error::WarNotFound(_) | Error::BattleNotFound(_) => {
                StatusCode::NOT_FOUND
            }
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
