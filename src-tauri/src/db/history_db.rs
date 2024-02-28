use chrono::{Date, DateTime, Utc};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

struct StateHistoryItem {
    id: String,
    time: DateTime<Utc>,
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
        todo!()
    }
}
