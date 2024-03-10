use app::db::{migrate, States};
use app::history::HistoryContext;
use app::settings::WorkCycleSettings;
use app::work_cycle::work_cycle_context::WorkCycleContext;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub fn init_test_environment() -> (
    WorkCycleContext,
    HistoryContext,
    Pool<SqliteConnectionManager>,
) {
    let settings = create_test_settings();
    let pool = init_test_database();
    let pool = pool.clone();
    let conn = pool.get().unwrap();
    let work_cycle_context = WorkCycleContext::new(settings, conn);

    let conn = pool.get().unwrap();
    let history_context = HistoryContext::new(conn);

    let pool = pool.clone();
    let mut conn = pool.get().unwrap();

    migrate(&mut conn);

    (work_cycle_context, history_context, pool)
}

pub fn get_all_states_from_db(
    connection: &PooledConnection<SqliteConnectionManager>,
) -> Vec<States> {
    let query = "SELECT id, state_id, started_time FROM states";
    let mut stmt = connection.prepare(query).unwrap();
    let states_iter = stmt
        .query_map([], |row| {
            Ok(States {
                id: row.get(0)?,
                state_id: row.get(1)?,
                started_time: row.get(2)?,
            })
        })
        .unwrap();

    states_iter.into_iter().map(|s| s.unwrap()).collect()
}

fn init_test_database() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::builder().build(manager).unwrap();
    pool
}
fn create_test_settings() -> WorkCycleSettings {
    let mut settings = WorkCycleSettings::new();
    settings.work_sessions_to_long_break = 3;
    settings
}
