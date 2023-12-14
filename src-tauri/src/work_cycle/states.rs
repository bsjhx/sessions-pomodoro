use crate::work_cycle::sessions_cycle::WorkCycle;

#[derive(Debug)]
pub struct NothingState;

#[derive(Debug)]
pub struct WorkingTimeState;

pub trait State {
    fn get_state_name(&self) -> String;

    fn start_cycle(self: Box<Self>) -> Box<dyn State>;
}

/// Starting point of application, initial value for cycle.
impl State for NothingState {
    fn get_state_name(&self) -> String {
        "NothingState".to_string()
    }

    /// Only this method matters in NothingState
    fn start_cycle(self: Box<Self>) -> Box<dyn State> {
        Box::new(WorkingTimeState)
    }
}

/// First state after cycle is started. It means, that user is working.
impl State for WorkingTimeState {
    fn get_state_name(&self) -> String {
        "WorkingTimeState".to_string()
    }
    /// Does nothing, can' start already started cycle
    fn start_cycle(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nothing_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(NothingState);
        let mut cycle = WorkCycle::new();

        // Act & Assert
        // let state = state.finish_cycle(&mut cycle);
        // assert_eq!(state.get_state_name(), "NothingState");

        // let state = state.end(&mut cycle);
        // assert_eq!(state.get_state_name(), "NothingState");
        //
        // let state = state.start_cycle();
        // assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn working_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(WorkingTimeState);
        let mut cycle = WorkCycle::new();

        // Act & Assert
        // let state = state.finish_cycle(&mut cycle);
        // assert_eq!(state.get_state_name(), "NothingState");
        //
        // let state = state.end(&mut cycle);
        // assert_eq!(state.get_state_name(), "NothingState");
        //
        // let state = state.start_cycle();
        // assert_eq!(state.get_state_name(), "WorkingTimeState");
    }
}
