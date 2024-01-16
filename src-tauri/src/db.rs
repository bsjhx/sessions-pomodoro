use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use mockall::automock;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[automock]
pub trait WorkingCycleDb {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>);
}

pub struct WorkingCycleDbSqliteImpl {
    connection: SqliteConnection,
}

impl WorkingCycleDb for WorkingCycleDbSqliteImpl {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>) {
        sql_query(format!(
            "insert into states(state_id, started_time) values ('worked! {}', '{}');",
            state_id, time
        ))
        .execute(&mut self.connection)
        .expect("TODO: panic message");
    }
}

pub fn init() -> SqliteConnection {
    if !db_file_exists() {
        create_db_file();
    }

    let mut connection = establish_connection();

    run_migrations(&mut connection);

    connection
}

fn run_migrations(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

fn establish_connection() -> SqliteConnection {
    let db_path = "sqlite://".to_string() + get_db_path().as_str();

    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    // TODO move this dir path to some global config
    home_dir.to_str().unwrap().to_string() + "/.config/sessions-pomodoro/database.sqlite"
}

#[cfg(test)]
pub(crate) mod common {
    use crate::db::MockWorkingCycleDb;

    pub fn get_mocked_working_cycle_trait() -> MockWorkingCycleDb {
        let mut mock = MockWorkingCycleDb::new();
        mock.expect_insert_state().returning(|_, _| ());

        mock
    }
}
