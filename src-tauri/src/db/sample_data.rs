use crate::db::db_init::Connection;
use crate::work_cycle::{
    LongBreakTimeState, NothingState, ShortBreakTimeState, StateId, WorkingTimeState,
};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;

pub fn insert_mock_data(connection: &Connection) {
    let mut time = Utc::now() - Duration::days(31);
    time = time.date_naive().and_hms_opt(7, 00, 00).unwrap().and_utc();
    let mut rng = rand::thread_rng();

    for _ in 0..32 {
        for _ in 0..rng.gen_range(1..5) {
            time = insert_finished_work_cycle(connection, time);
        }
        time += Duration::days(1);
        time = time.date_naive().and_hms_opt(7, 00, 00).unwrap().and_utc();
    }
}

fn insert_finished_work_cycle(connection: &Connection, start_time: DateTime<Utc>) -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    let number_of_states = rng.gen_range(2..10);
    let mut current_time = start_time;
    for i in 0..number_of_states {
        if i % 2 == 0 {
            insert_state(connection, WorkingTimeState::ID, current_time);
            current_time += Duration::seconds(25 * 60);
        } else {
            if i + 1 % 6 == 0 {
                insert_state(connection, LongBreakTimeState::ID, current_time);
                current_time += Duration::seconds(15 * 60);
            } else {
                insert_state(connection, ShortBreakTimeState::ID, current_time);
                current_time += Duration::seconds(5 * 60);
            }
        }
    }
    insert_state(connection, NothingState::ID, current_time);

    current_time += Duration::seconds(rng.gen_range(20..100) * 60);

    return current_time;
}

fn insert_state(connection: &Connection, state_id: &str, time: DateTime<Utc>) {
    connection
        .execute(
            "insert into states(state_id, started_time) values (?1, ?2);",
            (state_id, time),
        )
        .unwrap();
}
