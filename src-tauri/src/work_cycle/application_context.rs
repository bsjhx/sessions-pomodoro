use crate::configuration::WorkCycleSettings;
use crate::work_cycle::work_cycle_manager::StateHistoryElement;
use crate::work_cycle::{NothingState, State, WorkCycleManager};

pub struct ApplicationContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
    pub settings: WorkCycleSettings,
    pub work_cycle_manager: WorkCycleManager,
}

impl ApplicationContext {
    pub fn new(settings: WorkCycleSettings) -> Self {
        ApplicationContext {
            state: Some(Box::new(NothingState)),
            settings: WorkCycleSettings::default(),
            work_cycle_manager: WorkCycleManager::new(settings.work_sessions_to_long_break),
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
            self.state = Some(s.start_cycle(&mut self.work_cycle_manager))
        }
    }

    pub fn finish_cycle(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.finish_cycle(&mut self.work_cycle_manager))
        }
    }

    pub fn end_current_session(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.end(&mut self.work_cycle_manager))
        }
    }

    pub fn get_current_history(&self) -> &Vec<StateHistoryElement> {
        &self.work_cycle_manager.states_history
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        ApplicationContext::new(WorkCycleSettings::new())
    }
}
