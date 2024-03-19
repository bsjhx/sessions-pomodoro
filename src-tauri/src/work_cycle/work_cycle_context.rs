use crate::db::{WorkingCycleDb, WorkingCycleDbSqliteImpl};
use crate::settings::WorkCycleSettings;
use crate::work_cycle::{
    LongBreakTimeState, NothingState, ShortBreakTimeState, State, StateId, WorkCycleManager,
    WorkingTimeState,
};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct WorkCycleContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
    pub settings: WorkCycleSettings,
    pub work_cycle_manager: WorkCycleManager,
}

impl WorkCycleContext {
    pub fn new(
        settings: WorkCycleSettings,
        connection: PooledConnection<SqliteConnectionManager>,
    ) -> Self {
        let work_cycle_database = Box::new(WorkingCycleDbSqliteImpl::new(connection));

        let state: Option<Box<dyn State + Send + Sync>> =
            match work_cycle_database.fetch_last_state() {
                None => Some(Box::new(NothingState)),
                Some((id, _)) => match id.as_str() {
                    WorkingTimeState::ID => Some(Box::new(WorkingTimeState)),
                    ShortBreakTimeState::ID => Some(Box::new(WorkingTimeState)),
                    LongBreakTimeState::ID => Some(Box::new(WorkingTimeState)),
                    NothingState::ID => Some(Box::new(WorkingTimeState)),
                    _ => panic!("Wrong state in database."),
                },
            };

        WorkCycleContext {
            state,
            settings,
            work_cycle_manager: WorkCycleManager::new(
                settings.work_sessions_to_long_break,
                work_cycle_database,
            ),
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
}
