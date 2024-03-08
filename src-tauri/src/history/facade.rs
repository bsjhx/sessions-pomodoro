use crate::history::history_context::StateStatisticsDetails;
use crate::history::HistoryContext;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_today_states(state: State<Mutex<HistoryContext>>) -> StateStatisticsDetails {
    let history = state.lock().unwrap();

    history.get_states_history_for_today()
}
