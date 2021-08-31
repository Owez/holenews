//! Serde-based schemas to convert models such as battles into

use crate::map::LocationInfo;
use crate::models::{Battle, Population, War};
use crate::Result;
use serde::Serialize;
use sqlx::SqlitePool;

/// Top-level schema used as a basis for others, in order to unify constructs
#[derive(Serialize, Clone)]
pub struct Schema {
    pub wars: Option<Vec<SchemaWar>>,
    pub battles: Option<Vec<SchemaBattle>>,
}

impl Schema {
    /// Converts and adds a new war model
    pub fn add_war(mut self, war: War) -> Self {
        let new_war = SchemaWar::from(war);
        match self.wars {
            Some(wars) => wars.push(new_war),
            None => self.wars = Some(vec![new_war]),
        }
        self
    }

    /// Converts and adds multiple wars
    pub fn add_wars(mut self, wars: Vec<War>) -> Self {
        let mapped = wars.into_iter().map(|war| SchemaWar::from(war)).collect();
        match self.wars {
            Some(wars) => wars.extend(mapped),
            None => self.wars = Some(mapped),
        }
        self
    }

    /// Converts and adds a new battle model
    pub fn add_battle(mut self, battle: Battle) -> Self {
        let new_battle = SchemaBattle::from(battle);
        match self.battles {
            Some(battles) => battles.push(new_battle),
            None => self.battles = Some(vec![new_battle]),
        }
        self
    }

    /// Converts and adds multiple battles
    pub fn add_battles(mut self, battles: Vec<Battle>) -> Self {
        let mapped = battles
            .into_iter()
            .map(|battle| SchemaBattle::from(battle))
            .collect();
        match self.battles {
            Some(battles) => battles.extend(mapped),
            None => self.battles = Some(mapped),
        }
        self
    }

    /// Populates the `wars` part by all battles currently included
    #[allow(unused_mut)]
    pub async fn wars_from_battles(mut self, pool: &SqlitePool) -> Result<Self> {
        // TODO: make nicer algorithm for this
        let mut war_todos = vec![];
        match &self.battles {
            Some(battles) => {
                for battle in battles {
                    if !war_todos.contains(&battle.war_num) {
                        war_todos.push(battle.war_num)
                    }
                }
            }
            None => return Ok(self),
        }

        // TODO: figure out how to async launch all war_todos at same time
        let mut wars = vec![];
        for war_num in war_todos {
            wars.push(War::get_ensure(pool, war_num).await?)
        }
        Ok(self.add_wars(wars))
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self {
            wars: None,
            battles: None,
        }
    }
}

/// Conversion for a war model
#[derive(Serialize, Clone)]
pub struct SchemaWar {
    pub num: i64,
    pub time_start: String,
    pub time_end: String,
    pub colonial_win: Option<bool>,
    // pub submitted: String, // not needed
}

impl From<War> for SchemaWar {
    fn from(_war: War) -> Self {
        todo!("from war for schema")
    }
}

/// Conversion for a battle model
#[derive(Serialize, Clone)]
pub struct SchemaBattle {
    pub id: i64,
    pub war_num: i64,
    pub location_info: LocationInfo,
    pub name: String,
    pub description: Option<String>,
    pub last_edited: String,
    pub submitted: String,
    pub pop_reports: Vec<SchemaPopulation>,
}

impl From<Battle> for SchemaBattle {
    fn from(battle: Battle) -> Self {
        let pop_reports = SchemaPopulation::from_reports(battle.pop_reports);
        todo!("from battle for schema")
    }
}

/// Conversion for a population model; typically used as a vector of these
#[derive(Serialize, Clone)]
pub struct SchemaPopulation {
    pub counted: i64,
    pub at_time: String,
    pub description: Option<String>,
    pub last_edited: Option<String>,
    // pub submitted: String, // not needed
}

impl SchemaPopulation {
    pub fn from_reports(pop_reports: Option<Vec<Population>>) -> Option<Vec<Self>> {
        match pop_reports {
            Some(pop_reports) => Some(
                pop_reports
                    .into_iter()
                    .map(|pop_report| SchemaPopulation::from(pop_report))
                    .collect(),
            ),
            None => None,
        }
    }
}

impl From<Population> for SchemaPopulation {
    fn from(_pop_report: Population) -> Self {
        todo!("from population for schema")
    }
}
