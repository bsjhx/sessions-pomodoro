use crate::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
pub struct CurrentStateResponse {
    state_name: String,
    state_duration: i32,
}

#[tauri::command]
pub fn start_cycle(state: State<AppState>) -> CurrentStateResponse {
    let mut app = state.application_context.lock().unwrap();
    app.start_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
pub fn finish_cycle(state: State<AppState>) -> CurrentStateResponse {
    let mut app = state.application_context.lock().unwrap();
    app.finish_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
pub fn end_current_session(state: State<AppState>) -> CurrentStateResponse {
    let mut app = state.application_context.lock().unwrap();
    app.end_current_session();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
pub fn get_initial_time(state: State<AppState>) -> i32 {
    let app = state.application_context.lock().unwrap();
    app.settings.time_settings.working_time
}
