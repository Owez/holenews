//! Contains [Result], [Error] and implementations

use std::fmt;

/// Type cover for a result based on the [Error] variants
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-specific central error variants
pub enum Error {
    Database(sqlx::Error),
    LocationNotFound,
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
        }
    }
}
