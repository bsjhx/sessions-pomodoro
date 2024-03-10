use crate::db::{HistoryStatesDb, HistoryStatesDbSqliteImpl};
use crate::history::history_calculator::calculate;
use chrono::{DateTime, Utc};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct StatesDurationsDetails {
    total_length_in_minutes: i64,
    states: Vec<StateDurationDetails>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct StateDurationDetails {
    state_id: String,
    started_time: DateTime<Utc>,
    finished_time: DateTime<Utc>,
    length_in_seconds: i64,
}

impl StatesDurationsDetails {
    pub fn new(total_length: i64, states: Vec<StateDurationDetails>) -> Self {
        StatesDurationsDetails {
            total_length_in_minutes: total_length,
            states,
        }
    }
}

impl StateDurationDetails {
    pub fn new(
        id: &str,
        started_time: DateTime<Utc>,
        finished_time: DateTime<Utc>,
        length_in_seconds: i64,
    ) -> Self {
        StateDurationDetails {
            state_id: id.to_string(),
            started_time,
            finished_time,
            length_in_seconds,
        }
    }
}

pub struct HistoryContext {
    history_database: Box<dyn HistoryStatesDb + Send>,
}

impl HistoryContext {
    pub fn new(connection: PooledConnection<SqliteConnectionManager>) -> Self {
        HistoryContext {
            history_database: Box::new(HistoryStatesDbSqliteImpl::new(connection)),
        }
    }

    pub fn get_states_history_for_today(&self) -> StatesDurationsDetails {
        let today = chrono::offset::Utc::now();
        let states = self.history_database.get_states_history_by_date(today);
        calculate(&states)
    }
}
