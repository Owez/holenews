//! Contains [Result], [Error] and implementations

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::{fmt, io};

/// Type cover for a result based on the [Error] variants
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-specific central error variants
#[derive(Debug)]
pub enum Error {
    /// Static file opening error (most likely missing)
    StaticOpen(io::Error),
    /// Database error from sqlx
    Database(sqlx::Error),
    /// Map location couldn't be found
    LocationNotFound,
    /// War of number couldn't be found
    WarNotFound(i64),
    /// Battle of id couldn't be found
    BattleNotFound(i64),
    /// There was an error rendering templates using tera
    TemplateRender(tera::Error),
    /// Data provided for an input was too short
    DataTooShort,
    /// Data provided for an input was too long
    DataTooLong,
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
            Error::DataTooShort => write!(f, "Inputted data was too short"),
            Error::DataTooLong => write!(f, "Inputted data was too long"),
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
            Error::DataTooShort | Error::DataTooLong => StatusCode::BAD_REQUEST,
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
