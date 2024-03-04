use crate::helpers;
use crate::helpers::get_all_states_from_db;
use assertor::{assert_that, EqualityAssertion};
use chrono::{Duration, Timelike, Utc};
use rand::Rng;
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

    stmt.execute(params!["State1", today]).unwrap();
    stmt.execute(params!["State2", tomorrow]).unwrap();
    stmt.execute(params!["State3", yesterday]).unwrap();

    // When
    let today_states = history_context.get_states_history_for_today();

    // Then
    let states = get_all_states_from_db(&connection);
    assert_that!(states.len()).is_equal_to(3);

    assert_that!(today_states.len()).is_equal_to(1);
}

#[test]
fn history_context_should_return_today_states_for_random_data() {
    // Given
    let (_, history_context, pool) = helpers::init_test_environment();

    let pool = pool.clone();
    let connection = pool.get().unwrap();

    let query = "insert into states(state_id, started_time) values(?1, ?2)";
    let mut stmt = connection.prepare(query).unwrap();

    let now = Utc::now().with_hour(12).unwrap();

    let mut rng = rand::thread_rng();
    let mut today_counter = 0;
    for i in 0..100 {
        let state_id = format!("State{}", i);
        let days = rng.gen_range(-10..10);
        let date = now + Duration::days(days);

        if days == 0 {
            today_counter += 1;
        }

        stmt.execute(params![state_id, date]).unwrap();
    }

    // When
    let today_states = history_context.get_states_history_for_today();

    // Then
    let states = get_all_states_from_db(&connection);
    assert_that!(states.len()).is_equal_to(100);

    assert_that!(today_states.len()).is_equal_to(today_counter);
}
