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

        let last_updated: Option<DateTime<Utc>> = state_from_db
            .as_ref()
            .map(|(_, last_updated)| *last_updated);

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
        let time_since_state_started =
            (now - self.last_updated.unwrap_or(now)).num_seconds() as i32;
        let state_duration = self.get_current_state_duration();
        let mut time_left = state_duration - time_since_state_started;
        let mut overtime = if time_left < 0 { time_left.abs() } else { 0 };

        if time_left < 0 {
            time_left = state_duration;
        }

        if self.get_current_state_name() == NothingState::ID {
            time_left = state_duration;
            overtime = 0;
        }

        StateResponse {
            state_name: self.get_current_state_name(),
            is_runnable: self.state.as_ref().unwrap().is_runnable(),
            state_duration,
            overtime,
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
