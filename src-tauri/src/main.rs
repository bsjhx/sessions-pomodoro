// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::ApplicationContext;
use crate::work_cycle::facade::{end_current_session, finish_cycle, get_initial_time, start_cycle};
use std::sync::Mutex;

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

struct AppState {
    application_context: Mutex<ApplicationContext>,
}
