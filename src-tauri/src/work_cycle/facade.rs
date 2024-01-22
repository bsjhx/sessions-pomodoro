use crate::work_cycle::application_context::ApplicationContext;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize)]
pub struct CurrentStateResponse {
    state_name: String,
    state_duration: i32,
}

#[tauri::command]
pub fn start_cycle(state: State<Mutex<ApplicationContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    app.start_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
pub fn finish_cycle(state: State<Mutex<ApplicationContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    app.finish_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
pub fn end_current_session(state: State<Mutex<ApplicationContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    app.end_current_session();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
pub fn get_initial_time(state: State<Mutex<ApplicationContext>>) -> i32 {
    let app = state.lock().unwrap();
    app.settings.time_settings.working_time
}
