//! Contains [War] and implementations

use crate::{Error, Result};
use chrono::prelude::*;
use log::trace;
use sqlx::{FromRow, SqlitePool};

/// Ongoing or historic war
#[derive(FromRow)]
pub struct War {
    /// Number of war according to external sources
    pub num: i64,
    /// Start timestamp of this war
    pub time_start: NaiveDateTime,
    /// End timestamp of this war if it is historic
    pub time_end: Option<NaiveDateTime>,
    /// Indicates if the colonial faction won if it is historic
    pub colonial_win: Option<bool>,
    /// Timestamp of when this battle was submitted to the database
    pub submitted: NaiveDateTime,
}

impl War {
    /// Creates a new ongoing war and adds to database
    pub async fn new_ongoing(
        pool: &SqlitePool,
        num: i64,
        time_start: NaiveDateTime,
    ) -> Result<Self> {
        trace!("Adding new ongoing war of number {} to database", num);
        let submitted = Utc::now().naive_utc();

        sqlx::query!(
            "INSERT INTO war (num, time_start, submitted) VALUES (?, ?, ?)",
            num,
            time_start,
            submitted
        )
        .execute(pool)
        .await?;

        Ok(Self {
            num,
            time_start,
            time_end: None,
            colonial_win: None,
            submitted,
        })
    }

    /// Creates a new historic war and adds to database
    pub async fn new_historic(
        pool: &SqlitePool,
        num: i64,
        time_start: NaiveDateTime,
        time_end: NaiveDateTime,
        colonial_win: bool,
    ) -> Result<Self> {
        trace!("Adding new historic war of number {} to database", num);
        let submitted = Utc::now().naive_utc();

        sqlx::query!("INSERT INTO war (num, time_start, time_end, colonial_win, submitted) VALUES (?, ?, ?, ?, ?)", num, time_start, time_end, colonial_win, submitted).execute(pool).await?;

        Ok(Self {
            num,
            time_start,
            time_end: Some(time_end),
            colonial_win: Some(colonial_win),
            submitted,
        })
    }

    /// Attempts to get existing war from database
    pub async fn get(pool: &SqlitePool, num: i64) -> Result<Option<Self>> {
        trace!("Getting war of number {} from database", num);
        Ok(sqlx::query_as!(Self, "SELECT * FROM war WHERE num=?", num)
            .fetch_optional(pool)
            .await?)
    }

    /// Gets war from database, errors with not found compared to a normal get
    pub async fn get_ensure(pool: &SqlitePool, num: i64) -> Result<Self> {
        War::get(pool, num).await?.ok_or(Error::WarNotFound(num))
    }

    /// Updates provided values to update, does nothing if all values are none
    pub async fn update(
        pool: &SqlitePool,
        num: i64,
        time_end: Option<Option<NaiveDateTime>>,
        colonial_win: Option<Option<bool>>,
    ) -> Result<()> {
        trace!("Updating war of number {} in database", num);
        // TODO: refactor
        if let Some(time_end_val) = time_end {
            if let Some(colonial_win_val) = colonial_win {
                sqlx::query!(
                    "UPDATE war SET time_end=?, colonial_win=? WHERE num=?",
                    time_end_val,
                    colonial_win_val,
                    num
                )
                .execute(pool)
                .await?;
            } else {
                sqlx::query!("UPDATE war SET time_end=? WHERE num=?", time_end_val, num)
                    .execute(pool)
                    .await?;
            }
        } else if let Some(colonial_win_val) = colonial_win {
            sqlx::query!(
                "UPDATE war SET colonial_win=? WHERE num=?",
                colonial_win_val,
                num
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
