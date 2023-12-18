use crate::application_context::TimeSettings;
use crate::work_cycle::states::nothing_state::NothingState;
use crate::work_cycle::states::state_trait::State;
use crate::work_cycle::states::working_time_state::WorkingTimeState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BreakTimeState;

impl State for BreakTimeState {
    fn get_state_name(&self) -> String {
        "BreakTimeState".to_string()
    }

    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState)
    }

    fn end(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(WorkingTimeState)
    }

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        time_settings.break_time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn break_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(BreakTimeState);

        // Act & Assert - start and finish
        let state = state.start_cycle();
        assert_eq!(state.get_state_name(), "BreakTimeState");

        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        // Act & Assert - end
        let state = Box::new(BreakTimeState);
        let state = state.end();
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn break_time_should_return_proper_settings() {
        // Arrange
        let state = Box::new(BreakTimeState);
        let some_time_settings = TimeSettings {
            working_time: 100,
            break_time: 50,
        };

        // Act & Assert
        assert_eq!(state.get_state_name(), "BreakTimeState");
        assert_eq!(state.get_duration(&some_time_settings), 50);
    }
}