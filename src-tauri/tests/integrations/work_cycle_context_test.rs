use crate::helpers;
use crate::helpers::get_all_states_from_db;
use app::work_cycle::{LongBreakTimeState, NothingState, WorkingTimeState};
use app::work_cycle::{ShortBreakTimeState, StateId};
use assertor::{assert_that, EqualityAssertion};

#[test]
fn work_cycle_context_current_state_should_be_ok() {
    // Given
    let (mut work_cycle_context, _, pool) = helpers::init_test_environment();

    // When
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        NothingState::ID
    );

    work_cycle_context.start_cycle();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    work_cycle_context.end_current_session();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        ShortBreakTimeState::ID
    );

    work_cycle_context.end_current_session();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    work_cycle_context.end_current_session();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        ShortBreakTimeState::ID
    );

    work_cycle_context.end_current_session();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    work_cycle_context.end_current_session();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
        LongBreakTimeState::ID
    );

    work_cycle_context.finish_cycle();
    assert_eq!(
        work_cycle_context.get_current_state_name(),
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

#[test]
fn test_current_state_duration() {
    let (mut work_cycle_context, _, _) = helpers::init_test_environment();

    // Nothing state
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(0);

    // Working state
    work_cycle_context.start_cycle();
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(25 * 60);

    // Short break state
    work_cycle_context.end_current_session();
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(5 * 60);

    // Working state
    work_cycle_context.end_current_session();
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(25 * 60);

    // Short break state
    work_cycle_context.end_current_session();
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(5 * 60);

    // Working state
    work_cycle_context.end_current_session();
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(25 * 60);

    // Long break state
    work_cycle_context.end_current_session();
    assert_that!(work_cycle_context.get_current_state_duration()).is_equal_to(15 * 60);
}
