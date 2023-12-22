use crate::work_cycle::states::working_time_state::WorkingTimeState;
use crate::work_cycle::{State, WorkCycle};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NothingState;

/// Starting point of application, initial value for cycle.
impl State for NothingState {
    fn get_state_name(&self) -> String {
        "NothingState".to_string()
    }

    fn start_cycle(self: Box<Self>, _cycle: &mut WorkCycle) -> Box<dyn State + Send + Sync> {
        Box::new(WorkingTimeState)
    }

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        self
    }

    fn end(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::configuration::TimeSettings;
    use crate::work_cycle::states::nothing_state::NothingState;
    use crate::work_cycle::{State, WorkCycle};

    #[test]
    fn nothing_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(NothingState);
        let mut work_cycle = WorkCycle::new(4);

        // Act & Assert - finish and start
        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        // Act & Assert - end
        let state = Box::new(NothingState);

        let state = state.end();
        assert_eq!(state.get_state_name(), "NothingState");
    }

    #[test]
    fn nothing_state_should_return_proper_settings() {
        // Arrange
        let state = Box::new(NothingState);
        let some_time_settings = TimeSettings::new(100, 50, 75);

        // Act & Assert
        assert_eq!(state.get_state_name(), "NothingState");
        assert_eq!(state.get_duration(&some_time_settings), 0);
    }
}
