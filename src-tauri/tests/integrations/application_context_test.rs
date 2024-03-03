use crate::helpers;
use crate::helpers::get_all_states_from_db;
use app::db::States;
use app::work_cycle::{LongBreakTimeState, NothingState, WorkingTimeState};
use app::work_cycle::{ShortBreakTimeState, StateId};
use rusqlite::params;

#[test]
fn application_context_current_state_should_be_ok() {
    // Given
    let (mut application_context, _, pool) = helpers::init_test_environment();

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
    let pool = pool.clone();
    let connection = pool.get().unwrap();

    let results = get_all_states_from_db(&connection);

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
