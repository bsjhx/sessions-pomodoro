use app::db::init;
use assertor::{assert_that, BooleanAssertion, ResultAssertion};
use rusqlite::params;
use std::fs;
use std::path::Path;

const TEST_DB_FILE_PATH: &str = "./test-folder/test-db.sql";

struct After;

impl Drop for After {
    fn drop(&mut self) {
        fs::remove_file(TEST_DB_FILE_PATH).expect("Removing test database failed.");
    }
}

#[test]
fn test_init_database() {
    // After which removes test db on drop
    let _after = After;

    let pool = init(TEST_DB_FILE_PATH);

    // Test if file exists
    assert_that!(Path::new(TEST_DB_FILE_PATH).exists()).is_true();

    // Test if migration worked by checking if states table exists
    let pool = pool.clone();
    let connection = pool.get().unwrap();
    let query = "SELECT id, state_id, started_time FROM states";
    let mut stmt = connection.prepare(query).unwrap();
    let result = stmt.execute(params![]);
    assert_that!(result).is_ok();
}
