//! Contains [Battle] and implementations

use crate::map::*;
use crate::{models::Population, Error, Result};
use chrono::prelude::*;
use log::trace;
use sqlx::SqlitePool;

const MIN_NAME: usize = 5;
const MIN_DESCRIPTION: usize = 0;
const MAX_NAME: usize = 32;
const MAX_DESCRIPTION: usize = 2000;

/// Single battle in the history of foxhole
pub struct Battle {
    /// Id of battle, autoincremented
    pub id: i64,
    /// Number of war that this battle corresponds to
    pub war_num: i64,
    /// Map location this battle occurred in
    pub map: Map,
    /// Optional user-submitted name of this battle
    pub name: Option<String>,
    /// Optional user-submitted description of this battle
    pub description: Option<String>,
    /// When the last piece user-submitted content was edited, if any
    pub last_edited: Option<NaiveDateTime>,
    /// Timestamp of when this battle was submitted to the database
    pub submitted: NaiveDateTime,
    /// Population reports of this battle which may be fetched after making this structure
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
        trace!(
            "Adding new battle with war number {} at location {} from database",
            war_num,
            map_location
        );
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

    /// Attempts to get existing battle from database
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Self>> {
        trace!("Getting battle of id {} from database", id);
        let opt_record = sqlx::query!("SELECT * FROM battle WHERE id=?", id)
            .fetch_optional(pool)
            .await?;

        Ok(match opt_record {
            Some(record) => Some(Self {
                id: record.id,
                war_num: record.war_num,
                map: Map::from_name(&record.map_location).ok_or(Error::LocationNotFound)?,
                name: record.name,
                description: record.description,
                last_edited: record.last_edited,
                submitted: record.submitted,
                pop_reports: None,
            }),
            None => None,
        })
    }

    /// Gets battle from database, errors with not found compared to a normal get
    pub async fn get_ensure(pool: &SqlitePool, id: i64) -> Result<Self> {
        Self::get(pool, id).await?.ok_or(Error::BattleNotFound(id))
    }

    /// Gets top posts for homepage, typically ~10 in length; fully gets pop reports
    pub async fn get_homepage(pool: &SqlitePool) -> Result<Vec<Self>> {
        trace!("Getting homepage items from database");
        sqlx::query!("SELECT * FROM battle WHERE submitted >= datetime('now','-1 day')") // TODO: limit to 10-15
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
                    pop_reports: None,
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
        trace!("Updating battle of id {} in database", id);
        // TODO: refactor
        let name = name.into();
        let description = description.into();

        let last_edited = Utc::now().naive_utc();

        if let Some(name_val) = name {
            let name_len = name_val.len();
            if name_len > MAX_NAME {
                return Err(Error::DataTooLong);
            } else if name_len < MIN_NAME {
                return Err(Error::DataTooShort);
            } else if let Some(description_val) = description {
                let description_len = description_val.len();
                if description_len > MAX_DESCRIPTION {
                    return Err(Error::DataTooLong);
                } else if description_len < MIN_DESCRIPTION {
                    return Err(Error::DataTooShort);
                }

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
            let description_len = description_val.len();
            if description_len > MAX_DESCRIPTION {
                return Err(Error::DataTooLong);
            } else if description_len < MIN_DESCRIPTION {
                return Err(Error::DataTooShort);
            }

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

    /// Fetches all population reports related to this battles; chainable
    pub async fn get_pop_reports(mut self, pool: &SqlitePool) -> Result<Self> {
        trace!(
            "Getting pop reports for battle of id {} from database",
            self.id
        );
        self.pop_reports = Some(
            sqlx::query_as!(
                Population,
                "SELECT * FROM population WHERE battle_id=?",
                self.id
            )
            .fetch_all(pool)
            .await?,
        );
        Ok(self)
    }

    /// Generates a battle name automatically if a better one has not been assigned
    pub fn gen_name(&self) -> String {
        let map_name = self.map.name_friendly().1;
        let size = |num| match num {
            0..30 => "Skirmish",
            30..60 => "Clash",
            60..80 => "Battle",
            _ => "Onslaught",
        };
        let fmt = |sizetype| match self.submitted.hour() {
            0..2 | 22..24 => format!("The Midnight {} In {}", sizetype, map_name),
            2..6 => format!("{} At Daybreak: {} In Chaos", sizetype, map_name),
            6..19 => format!("The {} Of {}", sizetype, map_name),
            _ => format!("{} In {} At Dusk", sizetype, map_name),
        };

        match &self.pop_reports {
            Some(reports) if reports.len() != 0 => fmt(size(reports.last().unwrap().counted)),
            _ => format!("Breaking: Reports Of Fighting In {}", map_name),
        }
    }
}
