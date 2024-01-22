use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::fs;
use std::path::Path;

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
//
// pub fn init(db_path: &str) -> SqliteConnection {
//     if !db_file_exists(&db_path) {
//         create_db_file(&db_path);
//     }
//
//     let mut connection = establish_connection(&db_path);
//
//     run_migrations(&mut connection);
//
//     connection
// }
//
// fn run_migrations(connection: &mut SqliteConnection) {
//     connection.run_pending_migrations(MIGRATIONS).unwrap();
// }
//
// fn establish_connection(db_path: &str) -> SqliteConnection {
//     let db_path = format!("sqlite://{}", db_path);
//
//     SqliteConnection::establish(&db_path)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
// }
//
// fn create_db_file(db_path: &str) {
//     let db_dir = Path::new(db_path).parent().unwrap();
//
//     if !db_dir.exists() {
//         fs::create_dir_all(db_dir).unwrap();
//     }
//
//     fs::File::create(db_path).unwrap();
// }
//
// fn db_file_exists(db_path: &str) -> bool {
//     Path::new(&db_path).exists()
// }

#[cfg(test)]
pub(crate) mod common {
    use crate::db::MockWorkingCycleDb;

    pub fn get_mocked_working_cycle_trait() -> MockWorkingCycleDb {
        let mut mock = MockWorkingCycleDb::new();
        mock.expect_insert_state().returning(|_, _| ());

        mock
    }
}
