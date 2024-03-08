use crate::db::{HistoryStatesDb, HistoryStatesDbSqliteImpl};
use crate::history::history_calculator::calculate;
use chrono::{DateTime, Utc};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StateStatisticsDetails {
    pub total_length_in_minutes: i64,
    pub states: Vec<StateStatistics>,
}

#[derive(Debug, Serialize)]
pub struct StateStatistics {
    state_id: String,
    started_time: DateTime<Utc>,
    finished_time: DateTime<Utc>,
    length_in_seconds: i64,
}

impl StateStatisticsDetails {
    pub fn new(total_length: i64, states: Vec<StateStatistics>) -> Self {
        StateStatisticsDetails {
            total_length_in_minutes: total_length,
            states,
        }
    }
}

impl StateStatistics {
    pub fn new(
        id: &str,
        started_time: DateTime<Utc>,
        finished_time: DateTime<Utc>,
        length_in_seconds: i64,
    ) -> Self {
        StateStatistics {
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

    pub fn get_states_history_for_today(&self) -> StateStatisticsDetails {
        let today = chrono::offset::Utc::now();
        let states = self.history_database.get_states_history_by_date(today);
        calculate(&states)
    }
}
