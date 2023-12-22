// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::ApplicationContext;
use crate::configuration::WorkCycleSettings;
use crate::work_cycle::facade::{end_current_session, finish_cycle, get_initial_time, start_cycle};
use core::default::Default;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_store::StoreBuilder;

mod application_context;
mod configuration;
mod work_cycle;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let file_path = match env::var("POMODORO_FILES_PATH") {
                Ok(path) => path,
                Err(_) => {
                    panic!("POMODORO_FILES_PATH not set!!!")
                }
            };

            let mut path = PathBuf::new();
            path.push(file_path);
            path.push("dev_store");
            path.set_extension("json");

            let mut store = StoreBuilder::new(app.handle(), path).build();
            let _ = store.load();

            let a: State<AppState> = app.state();
            let mut a = a.application_context.lock().unwrap();

            let work_cycle_settings = store.get("workCycleSettings").unwrap().to_string();
            let work_cycle_settings: WorkCycleSettings =
                serde_json::from_str(&work_cycle_settings.to_string()).unwrap();

            a.settings = work_cycle_settings;

            Ok(())
        })
        .manage(AppState {
            application_context: Default::default(),
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
