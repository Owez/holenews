//! Contains [Battle] and implementations

use crate::map::*;
use crate::{models::Population, Error, Result};
use chrono::prelude::*;
use sqlx::SqlitePool;

/// Single battle in the history of foxhole
pub struct Battle {
    pub id: i64,
    pub war_num: i64,
    pub map: Map,
    pub name: Option<String>,
    pub description: Option<String>,
    pub last_edited: Option<NaiveDateTime>,
    pub submitted: NaiveDateTime,
    pub pop_reports: Option<Vec<Population>>,
}

impl Battle {
    /// Creates a new battle and adds to database
    pub async fn new(
        pool: &SqlitePool,
        war_num: i64,
        map_location: String,
        name: impl Into<Option<String>>,
        description: impl Into<Option<String>>,
    ) -> Result<Self> {
        let name = name.into();
        let map = Map::from_name(&map_location).ok_or(Error::LocationNotFound)?;
        let description = description.into();
        let submitted = Utc::now().naive_utc();
        let id = sqlx::query!("INSERT INTO battle (war_num, map_location, name, description, submitted) VALUES (?, ?, ?, ?, ?)", war_num, map_location, name, description,submitted).execute(pool).await?.last_insert_rowid();

        Ok(Self {
            id,
            war_num,
            map,
            name,
            description,
            last_edited: None,
            submitted,
            pop_reports: None,
        })
    }

    /// TODO: document
    pub async fn get(_pool: &SqlitePool, _id: i64) -> Result<Option<Self>> {
        todo!()
    }

    /// TODO: document
    pub async fn get_ensure(pool: &SqlitePool, id: i64) -> Result<Self> {
        Self::get(pool, id).await?.ok_or(Error::BattleNotFound(id))
    }

    /// Gets top posts for homepage, typically ~10 in length; fully gets pop reports
    pub async fn get_homepage(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query!("SELECT * FROM battle WHERE submitted >= datetime('now','-1 day')")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|record| {
                Ok(Self {
                    id: record.id,
                    war_num: record.war_num,
                    map: Map::from_name(&record.map_location).ok_or(Error::LocationNotFound)?,
                    name: record.name,
                    description: record.description,
                    last_edited: record.last_edited,
                    submitted: record.submitted,
                    pop_reports: None, // TODO: get pop reports
                })
            })
            .collect()
    }

    /// Updates provided values to update, does nothing if all values are none
    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        name: impl Into<Option<String>>,
        description: impl Into<Option<String>>,
    ) -> Result<()> {
        // TODO: refactor
        let name = name.into();
        let description = description.into();
        let last_edited = Utc::now().naive_utc();

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
    pub async fn get_pop_reports(&mut self, pool: &SqlitePool) -> Result<()> {
        self.pop_reports = Some(
            sqlx::query_as!(
                Population,
                "SELECT * FROM population WHERE battle_id=?",
                self.id
            )
            .fetch_all(pool)
            .await?,
        );
        Ok(())
    }

    /// Generates a battle name automatically if a better one has not been assigned
    pub fn gen_name(&self) -> String {
        todo!("generate battle name")
    }
}
