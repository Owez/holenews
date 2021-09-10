//! Contains [Population] and implementations

use crate::Result;
use chrono::prelude::*;
use log::trace;
use sqlx::{FromRow, SqlitePool};

/// Population report for a given battle, denoting a count for a given time
#[derive(FromRow)]
pub struct Population {
    /// Battle id this corresponds to
    pub battle_id: i64,
    /// Amount of population counted for this report
    pub counted: i64,
    /// Date at which this count occured, merged with the battle id as a primary key
    pub at_time: NaiveDateTime,
    /// Optional user-submitted description of this report
    pub description: Option<String>,
    /// When the last piece user-submitted content was edited, if any
    pub last_edited: Option<NaiveDateTime>,
    /// Timestamp of when this battle was submitted to the database
    pub submitted: NaiveDateTime,
}

impl Population {
    /// Adds a new population report to database via battle id
    pub async fn new(
        pool: &SqlitePool,
        battle_id: i64,
        counted: i64,
        at_time: NaiveDateTime,
        description: impl Into<Option<String>>,
    ) -> Result<Self> {
        trace!(
            "Adding new pop report with battle id of {} and time of {} to database",
            battle_id,
            at_time
        );
        let description = description.into();
        let submitted = Utc::now().naive_utc();

        sqlx::query!("INSERT INTO population (battle_id, counted, at_time, description, submitted) VALUES (?, ?, ?, ?, ?)", battle_id, counted, at_time, description, submitted).execute(pool).await?;

        Ok(Self {
            battle_id,
            counted,
            at_time,
            description,
            last_edited: None,
            submitted,
        })
    }

    /// Updates population report, typically used for descriptions
    pub async fn update(
        pool: &SqlitePool,
        battle_id: i64,
        at_time: NaiveDateTime,
        description: impl Into<Option<String>>,
    ) -> Result<()> {
        trace!(
            "Updating pop report with battle id of {} and time of {} in database",
            battle_id,
            at_time
        );
        let description = description.into();
        let last_edited = Utc::now().naive_utc();

        if let Some(desc_val) = description {
            sqlx::query!(
                "UPDATE population SET description=?, last_edited=? WHERE battle_id=? AND at_time=?",
                desc_val,
                last_edited,
                battle_id,
                at_time
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
