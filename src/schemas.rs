//! Serde-based schemas to convert models such as battles into

use crate::models::{Battle, Population, War};
use crate::{map::LocationInfo};
use serde::Serialize;

/// Top-level schema used as a basis for others, in order to unify constructs
#[derive(Serialize)]
pub struct SchemaTop {
    pub war: Option<SchemaWar>,
    pub battle: Option<SchemaBattle>,
}

impl SchemaTop {
    /// Converts and adds a new war model
    pub fn add_war(mut self, war: War) -> Self {
        self.war = Some(SchemaWar::from(war));
        self
    }

    /// Converts and adds a new battle model
    pub fn add_battle(mut self, battle: Battle) -> Self {
        self.battle = Some(SchemaBattle::from(battle));
        self
    }
}

impl Default for SchemaTop {
    fn default() -> Self {
        Self {
            war: None,
            battle: None,
        }
    }
}

/// Conversion for a war model
#[derive(Serialize)]
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
#[derive(Serialize)]
pub struct SchemaBattle {
    pub id: i64,
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
#[derive(Serialize)]
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
