use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::Migrations;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

pub fn migrate(connection: &mut PooledConnection<SqliteConnectionManager>) {
    println!("migracja");
    MIGRATIONS.to_latest(connection).unwrap();
    println!("migracja done");
}
