use chrono::{DateTime, TimeZone, Utc};
use std::fs;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use mockall::automock;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[automock]
pub trait WorkingCycleDb {
    fn insert_state(&self, state_id: String, time: DateTime<Utc>);
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
