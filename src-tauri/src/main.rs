// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application_context::ApplicationContext;
use crate::configuration::TimeSettings;
use crate::work_cycle::facade::{end_current_session, finish_cycle, get_initial_time, start_cycle};
use core::default::Default;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_store::{Store, StoreBuilder};

mod application_context;
mod configuration;
mod work_cycle;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let mut store = StoreBuilder::new(
                app.handle(),
                "/Users/michalrakoczy/Projects/rust/pomodoro-app/store.json"
                    .parse()
                    .unwrap(),
            )
            .build();
            let _ = store.load();

            let a: State<AppState> = app.state();
            let mut a = a.application_context.lock().unwrap();

            let time_settings = store.get("timeSettings").unwrap().to_string();
            let time_settings: TimeSettings =
                serde_json::from_str(&time_settings.to_string()).unwrap();

            a.time_settings = time_settings;

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
