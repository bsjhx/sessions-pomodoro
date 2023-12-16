use crate::work_cycle::{NothingState, State};

pub struct ApplicationContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        ApplicationContext {
            state: Some(Box::new(NothingState)),
        }
    }

    pub fn get_current_state_name(&self) -> String {
        self.state.as_ref().unwrap().get_state_name()
    }

    pub fn start_cycle(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.start_cycle())
        }
    }

    pub fn finish_cycle(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.finish_cycle())
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
