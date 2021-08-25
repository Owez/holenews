//! Contains [Battle] and implementations

use crate::Population;
use chrono::prelude::*;
use sqlx::SqlitePool;

/// Single battle in the history of foxhole
pub struct Battle {
    pub id: i64,
    pub war_num: i64,
    pub location: String, // TODO: use map.rs to make better
    pub name: Option<String>,
    pub description: Option<String>,
    pub last_edited: Option<DateTime<Utc>>,
    pub submitted: DateTime<Utc>,
    pub pop_reports: Option<Vec<Population>>,
}

impl Battle {
    /// Creates a new battle and adds to database
    pub async fn new(
        pool: &SqlitePool,
        war_num: i64,
        location: String,
        name: impl Into<Option<String>>,
        description: impl Into<Option<String>>,
    ) -> Result<Self, sqlx::Error> {
        let name = name.into();
        let description = description.into();
        let submitted = Utc::now();
        let id = sqlx::query!("INSERT INTO battle (war_num, location, name, description, submitted) VALUES (?, ?, ?, ?, ?)", war_num, location, name, description,submitted).execute(pool).await?.last_insert_rowid();

        Ok(Self {
            id,
            war_num,
            location,
            name,
            description,
            last_edited: None,
            submitted,
            pop_reports: None,
        })
    }

    /// Updates provided values to update, does nothing if all values are none
    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        name: impl Into<Option<String>>,
        description: impl Into<Option<String>>,
    ) -> Result<(), sqlx::Error> {
        // TODO: refactor
        let name = name.into();
        let description = description.into();
        let last_edited = Utc::now();

        if let Some(name_val) = name {
            if let Some(description_val) = description {
                sqlx::query!(
                    "UPDATE battle SET last_edited=?, name=?, description=? WHERE id=?",
                    last_edited,
                    name_val,
                    description_val,
                    id
                )
                .execute(pool)
                .await?;
            } else {
                sqlx::query!(
                    "UPDATE battle SET last_edited=?, name=? WHERE id=?",
                    last_edited,
                    name_val,
                    id
                )
                .execute(pool)
                .await?;
            }
        } else if let Some(description_val) = description {
            sqlx::query!(
                "UPDATE battle SET last_edited=?, description=? WHERE id=?",
                last_edited,
                description_val,
                id
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// Fetches all population reports related to this battles
    pub async fn get_pop_reports(&mut self, _pool: &SqlitePool) -> Result<(), sqlx::Error> {
        eprintln!(
            "Population report requested but sqlx is broken; {}",
            Utc::now()
        );
        // self.pop_reports = Some(
        //     sqlx::query_as!(
        //         Population,
        //         "SELECT * FROM population WHERE battle_id=?",
        //         self.id
        //     )
        //     .fetch_all(pool)
        //     .await?,
        // );
        Ok(())
    }
}
