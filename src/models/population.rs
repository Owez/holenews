//! Contains [Population] and implementations

use crate::Result;
use chrono::prelude::*;
use sqlx::{FromRow, SqlitePool};

/// Population report for a given battle, denoting a count for a given time
#[derive(FromRow)]
pub struct Population {
    pub battle_id: i64,
    pub counted: i64,
    pub at_time: NaiveDateTime,
    pub description: Option<String>,
    pub last_edited: Option<NaiveDateTime>,
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
