struct ApplicationContext {
    sessions_cycle: SessionsCycle,
    state: Box<dyn State>,
}

pub struct SessionsCycle {}

#[derive(Debug)]
pub struct NothingState;

#[derive(Debug)]
pub struct WorkingTimeState;

pub trait State {
    fn get_state_name(&self) -> String;
    fn start_cycle(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State>;
    fn finish_cycle(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State>;

    fn end(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State>;
}

/// Starting point of application, initial value for cycle.
impl State for NothingState {
    fn get_state_name(&self) -> String {
        "NothingState".to_string()
    }

    /// Only this method matters in NothingState
    fn start_cycle(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State> {
        Box::new(WorkingTimeState)
    }

    /// We can't finish not-started state
    fn finish_cycle(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State> {
        self
    }

    /// We can't end not-started state
    fn end(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State> {
        self
    }
}

/// First state after cycle is started. It means, that user is working.
impl State for WorkingTimeState {
    fn get_state_name(&self) -> String {
        "WorkingTimeState".to_string()
    }
    /// Does nothing, can' start already started cycle
    fn start_cycle(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State> {
        self
    }

    /// Finishes cycle, returns to nothing state.
    fn finish_cycle(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State> {
        Box::new(NothingState)
    }

    /// So far so good. It should head to break state
    fn end(self: Box<Self>, cycle: &mut SessionsCycle) -> Box<dyn State> {
        Box::new(NothingState)
    }
}

#[cfg(test)]
mod test {
    use crate::sessions::{NothingState, SessionsCycle, State, WorkingTimeState};

    #[test]
    fn nothing_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(NothingState);
        let mut cycle = SessionsCycle {};

        // Act & Assert
        let state = state.finish_cycle(&mut cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.end(&mut cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle(&mut cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn working_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(WorkingTimeState);
        let mut cycle = SessionsCycle {};

        // Act & Assert
        let state = state.finish_cycle(&mut cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.end(&mut cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle(&mut cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }
}
