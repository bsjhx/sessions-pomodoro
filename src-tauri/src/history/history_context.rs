use crate::db::{HistoryStatesDb, HistoryStatesDbSqliteImpl, StateHistoryItem};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct HistoryContext {
    history_database: Box<dyn HistoryStatesDb + Send>,
}

impl HistoryContext {
    pub fn new(connection: PooledConnection<SqliteConnectionManager>) -> Self {
        HistoryContext {
            history_database: Box::new(HistoryStatesDbSqliteImpl::new(connection)),
        }
    }

    pub fn get_states_history_for_today(&self) -> Vec<StateHistoryItem> {
        let today = chrono::offset::Utc::now();
        self.history_database.get_states_history_by_date(today)
    }
}
