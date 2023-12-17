use crate::work_cycle::{NothingState, State};
use serde::Serialize;

pub struct ApplicationContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
    pub time_settings: TimeSettings,
}

#[derive(Serialize, Copy, Clone)]
pub struct TimeSettings {
    #[serde(rename = "workingTime")]
    working_time: i32,
    #[serde(rename = "breakTime")]
    break_time: i32,
}

impl ApplicationContext {
    pub fn new() -> Self {
        let time_settings = TimeSettings {
            working_time: 25 * 60,
            break_time: 5 * 60,
        };

        ApplicationContext {
            state: Some(Box::new(NothingState)),
            time_settings,
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

    pub fn end_current_session(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.end())
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
