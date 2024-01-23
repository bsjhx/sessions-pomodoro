use crate::work_cycle::states::state_traits::StateId;
use crate::work_cycle::states::working_time_state::WorkingTimeState;
use crate::work_cycle::{State, WorkCycleManager};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NothingState;

impl NothingState {
    pub fn create_and_store(cycle: &mut WorkCycleManager) -> Self {
        cycle
            .on_state_changed(NothingState::ID.to_string())
            .expect("todo");
        NothingState
    }
}

impl StateId for NothingState {
    const ID: &'static str = "NothingState";
}

/// Starting point of application, initial value for cycle.
impl State for NothingState {
    fn get_state_name(&self) -> String {
        NothingState::ID.to_string()
    }

    fn start_cycle(self: Box<Self>, cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        Box::new(WorkingTimeState::create_and_store(cycle))
    }

    fn finish_cycle(
        self: Box<Self>,
        _cycle: &mut WorkCycleManager,
    ) -> Box<dyn State + Send + Sync> {
        self
    }

    fn end(self: Box<Self>, _cycle: &mut WorkCycleManager) -> Box<dyn State + Send + Sync> {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::db::get_mocked_working_cycle_trait;
    use crate::settings::TimeSettings;
    use crate::work_cycle::states::nothing_state::NothingState;
    use crate::work_cycle::{State, WorkCycleManager};

    #[test]
    fn nothing_state_should_be_able_to_change_state() {
        // Arrange
        let state = Box::new(NothingState);
        let mut work_cycle = WorkCycleManager::new(4, Box::new(get_mocked_working_cycle_trait()));

        // Act & Assert - finish and start
        let state = state.finish_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "NothingState");

        let state = state.start_cycle(&mut work_cycle);
        assert_eq!(state.get_state_name(), "WorkingTimeState");

        // Act & Assert - end
        let state = Box::new(NothingState);

        let state = state.end(&mut work_cycle);
        assert_eq!(state.get_state_name(), "NothingState");
    }

    #[test]
    fn nothing_state_should_return_proper_settings() {
        // Arrange
        let state = Box::new(NothingState);
        let some_time_settings = TimeSettings::new(100, 50, 75);

        // Act & Assert
        assert_eq!(state.get_state_name(), "NothingState");
        assert_eq!(state.get_duration(&some_time_settings), 0);
    }
}
