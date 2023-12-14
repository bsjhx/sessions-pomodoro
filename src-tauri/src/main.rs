// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::ApplicationContext;
use crate::work_cycle::{NothingState, WorkCycle};

mod application_context;
mod work_cycle;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_cycle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_cycle() -> String {
    let mut a = ApplicationContext::new();
    a.change_state();
    a.get_current_state_name()
}
