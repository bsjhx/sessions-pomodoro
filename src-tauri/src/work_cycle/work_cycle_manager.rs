use crate::db::WorkingCycleDb;
use chrono::Utc;

pub struct WorkCycleManager {
    work_sessions_until_long_break: u16,
    total_work_sessions_in_cycle: u16,
    work_cycle_database: Box<dyn WorkingCycleDb + Send>,
}

impl WorkCycleManager {
    pub fn new(
        work_sessions_until_long_break: u16,
        work_cycle_database: Box<dyn WorkingCycleDb + Send>,
    ) -> Self {
        WorkCycleManager {
            work_sessions_until_long_break,
            total_work_sessions_in_cycle: 0,
            work_cycle_database,
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
        self.work_cycle_database
            .insert_state(state_name, Utc::now());

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::db::MockWorkingCycleDb;
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
