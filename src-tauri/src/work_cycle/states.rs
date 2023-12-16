use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NothingState;

#[derive(Debug, Serialize)]
pub struct WorkingTimeState;

pub trait State {
    fn get_state_name(&self) -> String;

    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync>;

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync>;
}

/// Starting point of application, initial value for cycle.
impl State for NothingState {
    fn get_state_name(&self) -> String {
        "NothingState".to_string()
    }

    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(WorkingTimeState)
    }

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState)
    }
}

/// First state after cycle is started. It means, that user is working.
impl State for WorkingTimeState {
    fn get_state_name(&self) -> String {
        "WorkingTimeState".to_string()
    }
    /// Does nothing, can' start already started cycle
    ///
    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nothing_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(NothingState);

        // Act & Assert
        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle();
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn working_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(WorkingTimeState);

        // Act & Assert
        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle();
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }
}
