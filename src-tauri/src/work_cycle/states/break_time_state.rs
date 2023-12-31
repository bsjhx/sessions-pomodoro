use crate::configuration::TimeSettings;
use crate::work_cycle::states::nothing_state::NothingState;
use crate::work_cycle::states::state_trait::State;
use crate::work_cycle::states::working_time_state::WorkingTimeState;
use crate::work_cycle::WorkCycle;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ShortBreakTimeState;

// split to Long and Short break
impl State for ShortBreakTimeState {
    fn get_state_name(&self) -> String {
        "ShortBreakTimeState".to_string()
    }

    fn start_cycle(self: Box<Self>, _cycle: &mut WorkCycle) -> Box<dyn State + Send + Sync> {
        self
    }

    fn finish_cycle(self: Box<Self>) -> Box<dyn State + Send + Sync> {
        Box::new(NothingState)
    }

    fn end(self: Box<Self>, _cycle: &mut WorkCycle) -> Box<dyn State + Send + Sync> {
        Box::new(WorkingTimeState)
    }

    fn get_duration(&self, time_settings: &TimeSettings) -> i32 {
        time_settings.short_break_time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn break_time_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(ShortBreakTimeState);
        let mut work_cycle = WorkCycle::new(4);

        // Act & Assert - start and finish
        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "ShortBreakTimeState");

        let state = state.finish_cycle();
        assert_eq!(state.get_state_name(), "NothingState");

        // Act & Assert - end
        let state = Box::new(ShortBreakTimeState);
        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");
    }

    #[test]
    fn break_time_should_return_proper_settings() {
        // Arrange
        let state = Box::new(ShortBreakTimeState);
        let some_time_settings = TimeSettings::new(100, 50, 75);

        // Act & Assert
        assert_eq!(state.get_state_name(), "ShortBreakTimeState");
        assert_eq!(state.get_duration(&some_time_settings), 50);
    }
}
