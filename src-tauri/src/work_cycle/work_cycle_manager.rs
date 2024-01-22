use crate::db::db_init::WorkingCycleDb;
use chrono::Utc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct WorkCycleManager {
    work_sessions_until_long_break: u16,
    total_work_sessions_in_cycle: u16,
    pub states_history: Vec<StateHistoryElement>,
    c: Box<dyn WorkingCycleDb + Send>,
}

// TODO replace time with chrono::DateTime
pub struct StateHistoryElement {
    state_name: String,
    time: u64,
}

impl StateHistoryElement {
    pub fn new(state_name: String, time: u64) -> Self {
        StateHistoryElement { state_name, time }
    }

    pub fn get_name(&self) -> &str {
        &self.state_name
    }
}

impl WorkCycleManager {
    pub fn new(work_sessions_until_long_break: u16, c: Box<dyn WorkingCycleDb + Send>) -> Self {
        WorkCycleManager {
            work_sessions_until_long_break,
            total_work_sessions_in_cycle: 0,
            states_history: Vec::default(),
            c,
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
            state_name.clone(),
            since_the_epoch.as_secs(),
        ));

        self.c.insert_state(state_name, Utc::now());

        Ok(())
    }

    fn get_current_time_in_secs() -> Duration {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch
    }
}

#[cfg(test)]
mod test {
    use crate::db::db_init::MockWorkingCycleDb;
    use crate::work_cycle::WorkCycleManager;
    use assertor::{assert_that, BooleanAssertion};
    use rand::{thread_rng, Rng};

    #[test]
    fn after_n_work_sessions_next_break_should_be_long() {
        // Arrange
        let mut rng = thread_rng();
        let n = rng.gen_range(5..=20);
        let mut work_cycle = WorkCycleManager::new(n, Box::new(MockWorkingCycleDb::new()));

        // Act
        for _ in 0..n {
            assert_that!(work_cycle.is_next_break_long()).is_false();
            work_cycle.increment_work_session();
        }

        // Assert
        // After {n} finished work sessions, next break should be long
        assert_that!(work_cycle.is_next_break_long()).is_true();
    }
}
