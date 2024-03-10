use crate::history::history_context::StatesDurationsDetails;
use crate::history::HistoryContext;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn get_today_states(state: State<Mutex<HistoryContext>>) -> StatesDurationsDetails {
    let history = state.lock().unwrap();

    history.get_states_history_for_today()
}
