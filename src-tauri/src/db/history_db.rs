use chrono::{DateTime, Utc};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

pub struct StateHistoryItem {
    id: String,
    time: DateTime<Utc>,
}

impl StateHistoryItem {
    pub fn new(id: &str, time: DateTime<Utc>) -> Self {
        StateHistoryItem {
            id: id.to_string(),
            time,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_started_time(&self) -> &DateTime<Utc> {
        &self.time
    }
}

pub trait HistoryStatesDb {
    fn get_states_history_by_date(&self, date: DateTime<Utc>) -> Vec<StateHistoryItem>;
}

pub struct HistoryStatesDbSqliteImpl {
    connection: PooledConnection<SqliteConnectionManager>,
}

impl HistoryStatesDbSqliteImpl {
    pub fn new(connection: PooledConnection<SqliteConnectionManager>) -> Self {
        HistoryStatesDbSqliteImpl { connection }
    }
}

impl HistoryStatesDb for HistoryStatesDbSqliteImpl {
    fn get_states_history_by_date(&self, date: DateTime<Utc>) -> Vec<StateHistoryItem> {
        let from_midnight = date.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let to_midnight = date.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();
        let query =
            "select state_id, started_time from states where started_time between ?1 and ?2";
        let mut stmt = self.connection.prepare(query).unwrap();

        let result = stmt
            .query_map(params![from_midnight, to_midnight], |row| {
                Ok(StateHistoryItem {
                    id: row.get(0).unwrap_or_default(),
                    time: row.get(1).unwrap_or_default(),
                })
            })
            .unwrap();

        result.into_iter().map(|s| s.unwrap()).collect()
    }
}
