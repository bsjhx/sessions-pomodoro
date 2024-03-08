use crate::helpers;
use crate::helpers::get_all_states_from_db;
use app::work_cycle::{NothingState, StateId, WorkingTimeState};
use assertor::{assert_that, EqualityAssertion};
use chrono::{Duration, Timelike, Utc};
use rusqlite::params;

#[test]
fn history_context_should_return_today_states() {
    // Given
    let (_, history_context, pool) = helpers::init_test_environment();

    let pool = pool.clone();
    let connection = pool.get().unwrap();

    let query = "insert into states(state_id, started_time) values(?1, ?2)";
    let mut stmt = connection.prepare(query).unwrap();

    let today = Utc::now().with_hour(12).unwrap();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);

    stmt.execute(params![WorkingTimeState::ID, today]).unwrap();
    stmt.execute(params![NothingState::ID, today + Duration::seconds(50)])
        .unwrap();
    stmt.execute(params![WorkingTimeState::ID, tomorrow])
        .unwrap();
    stmt.execute(params![WorkingTimeState::ID, yesterday])
        .unwrap();

    // When
    let today_states = history_context.get_states_history_for_today();

    // Then
    let states = get_all_states_from_db(&connection);
    assert_that!(states.len()).is_equal_to(4);

    assert_that!(today_states.states.len()).is_equal_to(1);
    assert_that!(today_states.total_length_in_minutes).is_equal_to(50);
}
