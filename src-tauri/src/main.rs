// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::ApplicationContext;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

mod application_context;
mod work_cycle;
fn main() {
    tauri::Builder::default()
        .manage(AppState {
            application_context: Mutex::new(ApplicationContext::new()),
        })
        .invoke_handler(tauri::generate_handler![
            start_cycle,
            finish_cycle,
            end_current_session,
            get_initial_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
struct CurrentStateResponse {
    state_name: String,
    state_duration: i32,
}

#[tauri::command]
fn start_cycle(state: State<AppState>) -> CurrentStateResponse {
    let mut app = state.application_context.lock().unwrap();
    app.start_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
fn finish_cycle(state: State<AppState>) -> CurrentStateResponse {
    let mut app = state.application_context.lock().unwrap();
    app.finish_cycle();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
fn end_current_session(state: State<AppState>) -> CurrentStateResponse {
    let mut app = state.application_context.lock().unwrap();
    app.end_current_session();
    CurrentStateResponse {
        state_name: app.get_current_state_name(),
        state_duration: app.get_current_state_duration(),
    }
}

#[tauri::command]
fn get_initial_time(state: State<AppState>) -> i32 {
    let app = state.application_context.lock().unwrap();
    app.time_settings.working_time
}

struct AppState {
    application_context: Mutex<ApplicationContext>,
}
