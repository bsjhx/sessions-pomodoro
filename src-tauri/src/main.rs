// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::{ApplicationContext, TimeSettings};
use std::collections::HashMap;
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
            get_times
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_cycle(state: State<AppState>) -> String {
    let mut app = state.application_context.lock().unwrap();
    app.start_cycle();
    app.get_current_state_name()
}

#[tauri::command]
fn finish_cycle(state: State<AppState>) -> String {
    let mut app = state.application_context.lock().unwrap();
    app.finish_cycle();
    app.get_current_state_name()
}

#[tauri::command]
fn end_current_session(state: State<AppState>) -> String {
    let mut app = state.application_context.lock().unwrap();
    app.end_current_session();
    app.get_current_state_name()
}

#[tauri::command]
fn get_times(state: State<AppState>) -> TimeSettings {
    let app = state.application_context.lock().unwrap();
    app.time_settings
}

struct AppState {
    application_context: Mutex<ApplicationContext>,
}
