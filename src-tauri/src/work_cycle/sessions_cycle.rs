use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct WorkCycle {
    id: Uuid,
    last_state_change: u64,
    work_sessions_until_long_break: u16,
    total_work_sessions_in_cycle: u16,
}

impl WorkCycle {
    pub fn new(work_sessions_until_long_break: u16) -> Self {
        WorkCycle {
            id: Uuid::new_v4(),
            last_state_change: 0,
            work_sessions_until_long_break,
            total_work_sessions_in_cycle: 0,
        }
    }

    pub fn increment_work_session(&mut self) {
        self.total_work_sessions_in_cycle += 1;
    }

    pub fn is_next_break_long(&self) -> bool {
        self.total_work_sessions_in_cycle != 0
            && self.total_work_sessions_in_cycle % self.work_sessions_until_long_break == 0
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
    use crate::work_cycle::WorkCycle;
    use assertor::{assert_that, BooleanAssertion};
    use rand::Rng;

    #[test]
    fn after_n_work_sessions_next_break_should_be_long() {
        // Arrange
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(5..=20);
        let mut work_cycle = WorkCycle::new(n);

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
