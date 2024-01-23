use crate::settings::TimeSettings;
use crate::work_cycle::states::long_break_time_state::LongBreakTimeState;
use crate::work_cycle::states::nothing_state::NothingState;
use crate::work_cycle::states::state_traits::StateId;
use crate::work_cycle::{ShortBreakTimeState, State, WorkCycleManager};
use serde::Serialize;
use std::string::ToString;

#[derive(Debug, Serialize)]
pub struct WorkingTimeState;

impl WorkingTimeState {
    pub fn create_and_store(cycle: &mut WorkCycleManager) -> Self {
        cycle
            .on_state_changed(WorkingTimeState::ID.to_string())
            .expect("TODO: panic message");
        WorkingTimeState
    }
}

impl StateId for WorkingTimeState {
    const ID: &'static str = "WorkingTimeState";
}

/// First state after cycle is started. It means, that user is working.
impl State for WorkingTimeState {
    fn get_state_name(&self) -> String {
        WorkingTimeState::ID.to_string()
    }

    fn start_cycle(self: Box<Self>, _cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState::create_and_store(cycle))
    }

    fn end(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        cycle.increment_work_session();

        if cycle.is_next_break_long() {
            return Box::new(LongBreakTimeState::create_and_store(cycle));
        } else {
            Box::new(ShortBreakTimeState::create_and_store(cycle))
        }
    }

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        time_settings.working_time
    }
}

#[cfg(test)]
mod test {
    use crate::db::get_mocked_working_cycle_trait;
    use crate::settings::TimeSettings;
    use crate::work_cycle::states::working_time_state::WorkingTimeState;
    use crate::work_cycle::{State, WorkCycleManager};

    #[test]
    fn working_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(WorkingTimeState);
        let mut work_cycle = WorkCycleManager::new(4, Box::new(get_mocked_working_cycle_trait()));

        // Act & Assert - start and finish
        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        let state = state.finish_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        // Act & Assert - end
        let state = Box::new(WorkingTimeState);
        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "ShortBreakTimeState");
    }

    #[test]
    fn after_2_work_time_sessions_next_state_should_be_long_break() {
        // Arrange
        let state = Box::new(WorkingTimeState);
        let mut work_cycle = WorkCycleManager::new(2, Box::new(get_mocked_working_cycle_trait()));

        // Act
        // Start first working sessions
        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        // End first working sessions
        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "ShortBreakTimeState");

        // Start second working sessions
        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        // End second working sessions
        let state = state.end(&mut work_cycle);

        // Assert that after second working session there is long break
        assert_eq!(state.get_state_name(), "LongBreakTimeState");
    }

    #[test]
    fn working_time_should_return_proper_settings() {
        // Arrange
        let state = Box::new(WorkingTimeState);
        let some_time_settings = TimeSettings::new(100, 50, 75);

        // Act & Assert
        assert_eq!(state.get_state_name(), "WorkingTimeState");
        assert_eq!(state.get_duration(&some_time_settings), 100);
    }
}
