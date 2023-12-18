use crate::application_context::TimeSettings;

pub trait State {
    fn get_state_name(&self) -> String;

    fn start_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync>;

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync>;

    fn end(self: Box<Self>) -> Box<dyn State + Send + Sync>;

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        0
    }
}
