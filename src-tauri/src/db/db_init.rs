use std::fs;
use std::path::Path;

use crate::db::db_migrate::migrate;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn init(db_path: &str) -> Pool<SqliteConnectionManager> {
    if !db_file_exists(&db_path) {
        create_db_file(&db_path);
    }

    let pool = create_db_pool(&db_path);

    let pool = pool.clone();
    let mut connection = pool.get().unwrap();
    migrate(&mut connection);

    pool
}

fn create_db_pool(db_path: &str) -> Pool<SqliteConnectionManager> {
    // let db_path = format!("sqlite://{}", db_path);
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

fn db_file_exists(db_path: &str) -> bool {
    Path::new(&db_path).exists()
}
