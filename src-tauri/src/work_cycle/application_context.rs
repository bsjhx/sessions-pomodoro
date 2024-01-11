use crate::configuration::WorkCycleSettings;
use crate::work_cycle::{NothingState, State, WorkCycleManager};

pub struct ApplicationContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
    pub settings: WorkCycleSettings,
    pub current_work_cycle: WorkCycleManager,
}

impl ApplicationContext {
    pub fn new() -> Self {
        let settings = WorkCycleSettings::default();
        ApplicationContext {
            state: Some(Box::new(NothingState)),
            settings: WorkCycleSettings::default(),
            current_work_cycle: WorkCycleManager::new(settings.work_sessions_to_long_break),
        }
    }

    pub fn get_current_state_name(&self) -> String {
        self.state.as_ref().unwrap().get_state_name()
    }

    pub fn get_current_state_duration(&self) -> i32 {
        self.state
            .as_ref()
            .unwrap()
            .get_duration(&self.settings.time_settings)
    }

    pub fn start_cycle(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.start_cycle(&mut self.current_work_cycle))
        }
    }

    pub fn finish_cycle(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.finish_cycle(&mut self.current_work_cycle))
        }
    }

    pub fn end_current_session(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.end(&mut self.current_work_cycle))
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
