use crate::helpers;
use app::db::States;
use app::work_cycle::{LongBreakTimeState, NothingState, WorkingTimeState};
use app::work_cycle::{ShortBreakTimeState, StateId};

#[test]
fn application_context_current_state_should_be_ok() {
    // Given
    let (mut application_context, pool) = helpers::init_test_envirnment();

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
    let mut connection = pool.get().unwrap();

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
