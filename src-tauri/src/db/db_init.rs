use std::fs;
use std::path::Path;

use crate::db::db_migrate::migrate;
use crate::db::insert_mock_data;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type Connection = PooledConnection<SqliteConnectionManager>;

pub fn init(db_path: &str) -> Pool<SqliteConnectionManager> {
    if !db_file_exists(db_path) {
        create_db_file(db_path);
    }

    let pool = create_db_pool(db_path);

    let pool = pool.clone();
    let mut connection = pool.get().unwrap();
    migrate(&mut connection);

    pool
}

pub fn init_with_mock_data(db_path: &str) -> Pool<SqliteConnectionManager> {
    let already_exists = db_file_exists(db_path);
    let pool = init(db_path);
    let pool = pool.clone();
    let connection = pool.get().unwrap();

    if !already_exists {
        insert_mock_data(&connection);
    }

    pool
}

fn create_db_pool(db_path: &str) -> Pool<SqliteConnectionManager> {
    println!("Opening DB on path: [{}]", db_path);
    let manager = SqliteConnectionManager::file(db_path);
    Pool::builder().build(manager).unwrap()
}

fn create_db_file(db_path: &str) {
    let db_dir = Path::new(db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

pub fn db_file_exists(db_path: &str) -> bool {
    Path::new(&db_path).exists()
}
