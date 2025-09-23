use crate::db::{WorkingCycleDb, WorkingCycleDbSqliteImpl};
use crate::settings::WorkCycleSettings;
use crate::work_cycle::{
    LongBreakTimeState, NothingState, ShortBreakTimeState, State, StateId, StateResponse,
    WorkCycleManager, WorkingTimeState,
};
use chrono::{DateTime, Utc};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct WorkCycleContext {
    pub state: Option<Box<dyn State + Send + Sync>>,
    pub settings: WorkCycleSettings,
    pub work_cycle_manager: WorkCycleManager,
    pub last_updated: Option<DateTime<Utc>>,
}

impl WorkCycleContext {
    pub fn new(
        settings: WorkCycleSettings,
        connection: PooledConnection<SqliteConnectionManager>,
    ) -> Self {
        let work_cycle_database = Box::new(WorkingCycleDbSqliteImpl::new(connection));

        let state_from_db = work_cycle_database.fetch_last_state();
        let state: Option<Box<dyn State + Send + Sync>> = match &state_from_db {
            None => Some(Box::new(NothingState)),
            Some((id, _)) => match id.as_str() {
                WorkingTimeState::ID => Some(Box::new(WorkingTimeState)),
                ShortBreakTimeState::ID => Some(Box::new(ShortBreakTimeState)),
                LongBreakTimeState::ID => Some(Box::new(LongBreakTimeState)),
                NothingState::ID => Some(Box::new(NothingState)),
                _ => panic!("Wrong state in database. loaded from db=[{}]", id),
            },
        };

        let last_updated: Option<DateTime<Utc>> = match &state_from_db {
            None => None,
            Some((_, last_updated)) => Some(last_updated.clone()),
        };

        WorkCycleContext {
            state,
            settings,
            work_cycle_manager: WorkCycleManager::new(
                settings.work_sessions_to_long_break,
                work_cycle_database,
            ),
            last_updated,
        }
    }

    pub fn get_current_state(&self) -> StateResponse {
        let now: DateTime<Utc> = Utc::now();
        let diff = now - self.last_updated.unwrap_or(now);
        let mut time_left = self.get_current_state_duration() - diff.num_seconds() as i32;
        time_left = if time_left < 0 { 0 } else { time_left };
        StateResponse {
            state_name: self.get_current_state_name(),
            is_runnable: self.get_current_state_name() != NothingState::ID, // todo this should be loaded from settings as flag defined there
            time_left,
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
