use crate::work_cycle::work_cycle_context::WorkCycleContext;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize)]
pub struct CurrentStateResponse {
    state_name: String,
    state_duration: i32,
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn get_current_state(state: State<Mutex<WorkCycleContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn start_cycle(state: State<Mutex<WorkCycleContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    app.start_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn finish_cycle(state: State<Mutex<WorkCycleContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    app.finish_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn end_current_session(state: State<Mutex<WorkCycleContext>>) -> CurrentStateResponse {
    let mut app = state.lock().unwrap();
    app.end_current_session();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn get_initial_time(state: State<Mutex<WorkCycleContext>>) -> i32 {
    let app = state.lock().unwrap();
    app.settings.time_settings.working_time
}
