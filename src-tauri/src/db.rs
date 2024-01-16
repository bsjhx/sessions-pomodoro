use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::fs;
use std::path::Path;

use crate::schema::states::dsl::states;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use mockall::automock;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::states)]
pub struct States {
    pub id: Option<i32>,
    pub state_id: Option<String>,
    pub started_time: Option<NaiveDateTime>,
}

#[automock]
pub trait WorkingCycleDb {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>);
}

pub struct WorkingCycleDbSqliteImpl {
    connection: SqliteConnection,
}

impl WorkingCycleDbSqliteImpl {
    pub fn new(connection: SqliteConnection) -> Self {
        WorkingCycleDbSqliteImpl { connection }
    }
}

impl WorkingCycleDb for WorkingCycleDbSqliteImpl {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>) {
        sql_query(format!(
            "insert into states(state_id, started_time) values ('{}', '{}');",
            state_id, time
        ))
        .execute(&mut self.connection)
        .expect("TODO: panic message");
    }
}

pub fn init(db_path: &str) -> SqliteConnection {
    if !db_file_exists(&db_path) {
        create_db_file(&db_path);
    }

    let mut connection = establish_connection(&db_path);

    run_migrations(&mut connection);

    connection
}

fn run_migrations(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

fn establish_connection(db_path: &str) -> SqliteConnection {
    let db_path = format!("sqlite://{}", db_path);

    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

fn create_db_file(db_path: &str) {
    let db_dir = Path::new(db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists(db_path: &str) -> bool {
    Path::new(&db_path).exists()
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
