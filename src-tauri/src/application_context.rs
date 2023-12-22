use crate::configuration::TimeSettings;
use crate::work_cycle::{NothingState, State, WorkCycle};

pub struct ApplicationContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
    pub time_settings: TimeSettings,
    pub current_work_cycle: Option<WorkCycle>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        ApplicationContext {
            state: Some(Box::new(NothingState)),
            time_settings: TimeSettings::default(),
            current_work_cycle: None,
        }
    }

    pub fn get_current_state_name(&self) -> String {
        self.state.as_ref().unwrap().get_state_name()
    }

    pub fn get_current_state_duration(&self) -> i32 {
        self.state
            .as_ref()
            .unwrap()
            .get_duration(&self.time_settings)
    }

    pub fn start_cycle(&mut self) {
        self.current_work_cycle = Some(WorkCycle::new(4));

        if let Some(s) = self.state.take() {
            self.state = Some(s.start_cycle(&mut self.current_work_cycle.as_mut().unwrap()))
        }
    }

    pub fn finish_cycle(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.finish_cycle())
        }
    }

    pub fn end_current_session(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.end(&mut self.current_work_cycle.as_mut().unwrap()))
        }
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        ApplicationContext::new()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(42, 42);
    }
}
