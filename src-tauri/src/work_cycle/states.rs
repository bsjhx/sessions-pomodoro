use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NothingState;

#[derive(Debug, Serialize)]
pub struct WorkingTimeState;

#[derive(Debug, Serialize)]
pub struct BreakTimeState;

pub trait State {
    fn get_state_name(&self) -> String;

    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync>;

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync>;

    fn end(self: Box<Self>) -> Box<dyn State + Send + Sync>;
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
        self
    }

    fn end(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        self
    }
}

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
}

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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nothing_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(NothingState);

        // Act & Assert - finish and start
        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle();
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        // Act & Assert - end
        let state = Box::new(NothingState);

        let state = state.end();
        assert_eq!(state.get_state_name(), "NothingState");
    }

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
}
