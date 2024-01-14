// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::__cmd__finish_cycle;
use app::__cmd__get_initial_time;
use app::__cmd__start_cycle;
use app::configuration::WorkCycleSettings;
use app::work_cycle::application_context::ApplicationContext;
use app::work_cycle::facade::{end_current_session, finish_cycle, get_initial_time, start_cycle};
use app::{__cmd__end_current_session, db};
use core::default::Default;
use diesel::RunQueryDsl;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // ***** DATABASE *****
            let connection = db::init();
            app.manage(Mutex::new(connection));
            // ***** DATABASE *****

            // ***** SETTINGS FILE *****
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
            // ***** SETTINGS FILE *****

            let mut store = StoreBuilder::new(app.handle(), path).build();
            let _ = store.load();

            let work_cycle_settings = store.get("workCycleSettings").unwrap().to_string();
            let work_cycle_settings: WorkCycleSettings =
                serde_json::from_str(&work_cycle_settings.to_string()).unwrap();

            let app_context = ApplicationContext::new(work_cycle_settings);
            app.manage(Mutex::new(app_context));

            Ok(())
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
