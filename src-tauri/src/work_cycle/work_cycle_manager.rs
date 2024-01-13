use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct WorkCycleManager {
    id: Uuid,
    last_state_change: u64,
    work_sessions_until_long_break: u16,
    total_work_sessions_in_cycle: u16,
    states_history: Vec<StateHistoryElement>,
}

// TODO replace time with chrono::DateTime
struct StateHistoryElement {
    state_name: String,
    time: u64,
}

impl StateHistoryElement {
    pub fn new(state_name: String, time: u64) -> Self {
        StateHistoryElement { state_name, time }
    }
}

impl WorkCycleManager {
    pub fn new(work_sessions_until_long_break: u16) -> Self {
        WorkCycleManager {
            id: Uuid::new_v4(),
            last_state_change: 0,
            work_sessions_until_long_break,
            total_work_sessions_in_cycle: 0,
            states_history: Vec::default(),
        }
    }

    pub fn increment_work_session(&mut self) {
        self.total_work_sessions_in_cycle += 1;
    }

    pub fn is_next_break_long(&self) -> bool {
        self.total_work_sessions_in_cycle != 0
            && self.total_work_sessions_in_cycle % self.work_sessions_until_long_break == 0
    }

    pub fn on_state_changed(&mut self, state_name: String) -> Result<(), String> {
        let since_the_epoch = Self::get_current_time_in_secs();

        self.states_history.push(StateHistoryElement::new(
            state_name,
            since_the_epoch.as_secs(),
        ));

        Ok(())
    }

    fn get_current_time_in_secs() -> Duration {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch
    }

    pub fn update_last_state_change(&mut self) {
        self.last_state_change = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    }
}

#[cfg(test)]
mod test {
    use crate::work_cycle::WorkCycleManager;
    use assertor::{assert_that, BooleanAssertion};
    use chrono::Utc;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use serde::de::Unexpected::Str;

    #[test]
    fn after_n_work_sessions_next_break_should_be_long() {
        // Arrange
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(5..=20);
        let mut work_cycle = WorkCycleManager::new(n);

        // Act
        for _ in 0..n {
            assert_that!(work_cycle.is_next_break_long()).is_false();
            work_cycle.increment_work_session();
        }

        // Assert
        // After {n} finished work sessions, next break should be long
        assert_that!(work_cycle.is_next_break_long()).is_true();
    }

    #[test]
    fn added_new_states_should_stored_in_vector() {
        let mut wcm = WorkCycleManager::new(4);
        let mut added_states = vec![];

        for i in 0..50 {
            let random_state_name: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();

            added_states.push(random_state_name.clone());
            assert_eq!(wcm.on_state_changed(random_state_name), Ok(()));
        }

        for (i, state) in wcm.states_history.iter().enumerate() {
            assert_eq!(state.state_name, added_states[i]);
        }
    }
}
