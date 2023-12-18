use crate::application_context::TimeSettings;
use crate::work_cycle::states::nothing_state::NothingState;
use crate::work_cycle::{BreakTimeState, State};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WorkingTimeState;

/// First state after cycle is started. It means, that user is working.
impl State for WorkingTimeState {
    fn get_state_name(&self) -> String {
        "WorkingTimeState".to_string()
    }

    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState)
    }

    fn end(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(BreakTimeState)
    }

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        time_settings.working_time
    }
}

#[cfg(test)]
mod test {
    use crate::work_cycle::states::working_time_state::WorkingTimeState;
    use crate::work_cycle::State;

    #[test]
    fn working_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(WorkingTimeState);

        // Act & Assert - start and finish
        let state = state.start_cycle();
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        // Act & Assert - end
        let state = Box::new(WorkingTimeState);
        let state = state.end();
        assert_eq!(state.get_state_name(), "BreakTimeState");
    }
}
