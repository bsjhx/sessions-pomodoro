use app::configuration::WorkCycleSettings;
use app::db;
use app::work_cycle::application_context::ApplicationContext;
use app::work_cycle::{LongBreakTimeState, NothingState, WorkingTimeState};
use app::work_cycle::{ShortBreakTimeState, StateId};
use diesel::{connection, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[test]
fn application_context_current_state_should_be_ok() {
    // Given
    let settings = create_test_settings();
    let connection = init_test_database();
    let mut application_context = ApplicationContext::new(settings, connection);

    // When & then
    assert_eq!(
        application_context.get_current_state_name(),
        NothingState::ID
    );

    application_context.start_cycle();
    assert_eq!(
        application_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        ShortBreakTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        ShortBreakTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        LongBreakTimeState::ID
    );

    application_context.finish_cycle();
    assert_eq!(
        application_context.get_current_state_name(),
        NothingState::ID
    );
}

#[test]
fn application_context_should_keep_history() {
    // Given
    let settings = create_test_settings();
    let connection = init_test_database();
    let mut application_context = ApplicationContext::new(settings, connection);

    let mut states_expected = vec![];
    states_expected.push(WorkingTimeState::ID);
    states_expected.push(ShortBreakTimeState::ID);
    states_expected.push(WorkingTimeState::ID);
    states_expected.push(ShortBreakTimeState::ID);
    states_expected.push(WorkingTimeState::ID);
    states_expected.push(LongBreakTimeState::ID);
    states_expected.push(NothingState::ID);

    // When & then
    application_context.start_cycle();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.finish_cycle();

    for (i, actual_state) in application_context.get_current_history().iter().enumerate() {
        assert_eq!(actual_state.get_name(), states_expected[i]);
    }
}

fn init_test_database() -> SqliteConnection {
    let mut connection = SqliteConnection::establish(":memory:")
        .unwrap_or_else(|_| panic!("Error creating test database"));
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Test migration failed");

    connection
}
fn create_test_settings() -> WorkCycleSettings {
    let mut settings = WorkCycleSettings::new();
    settings.work_sessions_to_long_break = 3;
    settings
}