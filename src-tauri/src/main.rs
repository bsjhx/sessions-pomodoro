// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::ApplicationContext;
use crate::work_cycle::{NothingState, WorkCycle};
use std::sync::Mutex;
use tauri::State;

mod application_context;
mod work_cycle;
fn main() {
    tauri::Builder::default()
        .manage(AppState {
            application_context: Mutex::new(ApplicationContext::new()),
        })
        .invoke_handler(tauri::generate_handler![start_cycle, finish_cycle])
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

struct AppState {
    application_context: Mutex<ApplicationContext>,
}
