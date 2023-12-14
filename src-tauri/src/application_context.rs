use crate::work_cycle::{NothingState, State, WorkCycle};
use std::mem;

pub struct ApplicationContext {
    current_work_cycle: WorkCycle,
    state: Option<Box<dyn State>>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        ApplicationContext {
            current_work_cycle: WorkCycle::new(),
            state: Some(Box::new(NothingState)),
        }
    }

    pub fn get_current_state_name(&self) -> String {
        self.state.as_ref().unwrap().get_state_name()
    }

    pub fn change_state(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.start_cycle())
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(42, 42);
    }
}
