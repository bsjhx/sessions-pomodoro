use crate::history::history_context::StatesDurationsDetails;
use crate::history::HistoryContext;
use chrono::{DateTime, Utc};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
#[cfg(not(wtarpaulin_include))]
pub fn get_states_for_day(
    state: State<Mutex<HistoryContext>>,
    day: DateTime<Utc>,
) -> StatesDurationsDetails {
    let history = state.lock().unwrap();
    history.get_states_history_for_day(day)
}
