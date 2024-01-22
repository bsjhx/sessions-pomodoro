use crate::configuration::TimeSettings;
use crate::work_cycle::states::nothing_state::NothingState;
use crate::work_cycle::states::state_traits::{State, StateId};
use crate::work_cycle::states::working_time_state::WorkingTimeState;
use crate::work_cycle::WorkCycleManager;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ShortBreakTimeState;

impl StateId for ShortBreakTimeState {
    const ID: &'static str = "ShortBreakTimeState";
}

impl State for ShortBreakTimeState {
    fn get_state_name(&self) -> String {
        ShortBreakTimeState::ID.to_string()
    }

    fn start_cycle(self: Box<Self>, _cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        cycle
            .on_state_changed(NothingState::ID.to_string())
            .expect("TODO: panic message");

        Box::new(NothingState)
    }

    fn end(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        cycle
            .on_state_changed(WorkingTimeState::ID.to_string())
            .expect("TODO: panic message");

        Box::new(WorkingTimeState)
    }

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        time_settings.short_break_time
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::get_mocked_working_cycle_trait;

    #[test]
    fn break_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(ShortBreakTimeState);
        let mut work_cycle = WorkCycleManager::new(4, Box::new(get_mocked_working_cycle_trait()));

        // Act & Assert - start and finish
        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "ShortBreakTimeState");

        let state = state.finish_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        // Act & Assert - end
        let state = Box::new(ShortBreakTimeState);
        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn break_time_should_return_proper_settings() {
        // Arrange
        let state = Box::new(ShortBreakTimeState);
        let some_time_settings = TimeSettings::new(100, 50, 75);

        // Act & Assert
        assert_eq!(state.get_state_name(), "ShortBreakTimeState");
        assert_eq!(state.get_duration(&some_time_settings), 50);
    }
}
