use chrono::{DateTime, Utc};
use mockall::automock;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct States {
    pub id: Option<i32>,
    pub state_id: Option<String>,
    pub started_time: Option<DateTime<Utc>>,
}

#[automock]
pub trait WorkingCycleDb {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>);
}

pub struct WorkingCycleDbSqliteImpl {
    connection: PooledConnection<SqliteConnectionManager>,
}

impl WorkingCycleDbSqliteImpl {
    pub fn new(connection: PooledConnection<SqliteConnectionManager>) -> Self {
        WorkingCycleDbSqliteImpl { connection }
    }
}

impl WorkingCycleDb for WorkingCycleDbSqliteImpl {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>) {
        self.connection
            .execute(
                "insert into states(state_id, started_time) values (?1, ?2);",
                (state_id, time),
            )
            .unwrap();
    }
}

#[cfg(test)]
pub mod common {
    use crate::db::working_cycle_db::MockWorkingCycleDb;

    pub fn get_mocked_working_cycle_trait() -> MockWorkingCycleDb {
        let mut mock = MockWorkingCycleDb::new();
        mock.expect_insert_state().returning(|_, _| ());

        mock
    }
}
