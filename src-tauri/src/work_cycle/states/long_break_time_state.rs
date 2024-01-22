use crate::configuration::TimeSettings;
use crate::work_cycle::states::nothing_state::NothingState;
use crate::work_cycle::states::state_traits::{State, StateId};
use crate::work_cycle::states::working_time_state::WorkingTimeState;
use crate::work_cycle::WorkCycleManager;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LongBreakTimeState;

impl LongBreakTimeState {
    pub fn create_and_store(cycle: &mut WorkCycleManager) -> Self {
        cycle
            .on_state_changed(LongBreakTimeState::ID.to_string())
            .expect("todo");
        LongBreakTimeState
    }
}

impl StateId for LongBreakTimeState {
    const ID: &'static str = "LongBreakTimeState";
}

impl State for LongBreakTimeState {
    fn get_state_name(&self) -> String {
        LongBreakTimeState::ID.to_string()
    }

    fn start_cycle(self: Box<Self>, _cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState::create_and_store(cycle))
    }

    fn end(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        Box::new(WorkingTimeState::create_and_store(cycle))
    }

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        time_settings.long_break_time
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::get_mocked_working_cycle_trait;

    #[test]
    fn long_break_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(LongBreakTimeState);
        let mut work_cycle = WorkCycleManager::new(4, Box::new(get_mocked_working_cycle_trait()));

        // Act & Assert - start and finish
        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "LongBreakTimeState");

        let state = state.finish_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        // Act & Assert - end
        let state = Box::new(LongBreakTimeState);
        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn long_break_time_should_return_proper_settings() {
        // Arrange
        let state = Box::new(LongBreakTimeState);
        let some_time_settings = TimeSettings::new(100, 50, 75);

        // Act & Assert
        assert_eq!(state.get_state_name(), "LongBreakTimeState");
        assert_eq!(state.get_duration(&some_time_settings), 75);
    }
}
