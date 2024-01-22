use app::configuration::WorkCycleSettings;
use app::db::States;
use app::work_cycle::application_context::ApplicationContext;
use app::work_cycle::{LongBreakTimeState, NothingState, WorkingTimeState};
use app::work_cycle::{ShortBreakTimeState, StateId};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[test]
fn application_context_current_state_should_be_ok() {
    // Given
    let settings = create_test_settings();
    let mut pool = init_test_database();
    let pool = pool.clone();
    let conn = pool.get().unwrap();
    let mut application_context = ApplicationContext::new(settings, conn);

    let pool = pool.clone();
    let conn = pool.get().unwrap();

    conn.execute(
        "create table states (
    id integer primary key,
    state_id tinytext,
    started_time datetime
);",
        (),
    )
    .unwrap();

    // When
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

    // Then
    let query = "SELECT id, state_id, started_time FROM states";
    let mut stmt = conn.prepare(query).unwrap();
    let states_iter = stmt
        .query_map([], |row| {
            Ok(States {
                id: row.get(0)?,
                state_id: row.get(1)?,
                started_time: row.get(2)?,
            })
        })
        .unwrap();

    let results: Vec<States> = states_iter.into_iter().map(|s| s.unwrap()).collect();

    assert_eq!(results.len(), 7);

    let expected_states = vec![
        (1, WorkingTimeState::ID),
        (2, ShortBreakTimeState::ID),
        (3, WorkingTimeState::ID),
        (4, ShortBreakTimeState::ID),
        (5, WorkingTimeState::ID),
        (6, LongBreakTimeState::ID),
        (7, NothingState::ID),
    ];

    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.id, Some(expected_states[i].0));
        assert_eq!(result.state_id, Some(expected_states[i].1.to_string()));
    }
}

fn init_test_database() -> Pool<SqliteConnectionManager> {
    let manager = r2d2_sqlite::SqliteConnectionManager::memory();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}
fn create_test_settings() -> WorkCycleSettings {
    let mut settings = WorkCycleSettings::new();
    settings.work_sessions_to_long_break = 3;
    settings
}
