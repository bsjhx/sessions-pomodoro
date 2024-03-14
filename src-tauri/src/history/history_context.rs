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
    running_state: Option<RunningStateDetails>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct StateDurationDetails {
    state_id: String,
    started_time: DateTime<Utc>,
    finished_time: DateTime<Utc>,
    length_in_seconds: i64,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct RunningStateDetails {
    state_id: String,
    started_time: DateTime<Utc>,
}

impl StatesDurationsDetails {
    pub fn new(total_length: i64, states: Vec<StateDurationDetails>) -> Self {
        StatesDurationsDetails {
            total_length_in_minutes: total_length,
            states,
            running_state: None,
        }
    }

    pub fn new_with_running_state(
        total_length: i64,
        states: Vec<StateDurationDetails>,
        running_state: RunningStateDetails,
    ) -> Self {
        StatesDurationsDetails {
            total_length_in_minutes: total_length,
            states,
            running_state: Some(running_state),
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

impl RunningStateDetails {
    pub fn new(id: &str, started_time: DateTime<Utc>) -> Self {
        RunningStateDetails {
            state_id: id.to_string(),
            started_time,
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

    pub fn get_states_history_for_day(&self, day: DateTime<Utc>) -> StatesDurationsDetails {
        let states = self.history_database.get_states_history_by_date(day);
        calculate(&states)
    }
}
