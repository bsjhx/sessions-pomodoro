use app::configuration::WorkCycleSettings;
use app::db::migrate;
use app::work_cycle::application_context::ApplicationContext;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn init_test_envirnment() -> (ApplicationContext, Pool<SqliteConnectionManager>) {
    let settings = create_test_settings();
    let pool = init_test_database();
    let pool = pool.clone();
    let conn = pool.get().unwrap();
    let mut application_context = ApplicationContext::new(settings, conn);

    let pool = pool.clone();
    let mut conn = pool.get().unwrap();

    migrate(&mut conn);

    (application_context, pool)
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
