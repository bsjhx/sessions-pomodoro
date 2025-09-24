use crate::settings::TimeSettings;
use crate::work_cycle::WorkCycleManager;

pub trait State {
    fn get_state_name(&self) -> String;

    fn start_cycle(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync>;

    fn finish_cycle(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync>;

    fn end(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync>;

    fn get_duration(&self, _time_settings: &TimeSettings) -> i32 {
        0
    }

    fn is_runnable(&self) -> bool {
        false
    }
}

pub trait StateId {
    const ID: &'static str;
}
