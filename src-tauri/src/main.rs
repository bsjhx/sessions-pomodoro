// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::__cmd__finish_cycle;
use app::__cmd__get_initial_time;
use app::__cmd__start_cycle;
use app::configuration::WorkCycleSettings;
use app::work_cycle::facade::AppState;
use app::work_cycle::facade::{end_current_session, finish_cycle, get_initial_time, start_cycle};
use app::{__cmd__end_current_session, db};
use core::default::Default;
use diesel::{sql_query, RunQueryDsl};
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_store::StoreBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // ***** DATABASE *****
            let mut connection = db::init();

            // sql_query("insert into states(state_id, started_time) values ('from here', '2024-01-14 16:52:88');")
            //     .execute(&mut connection)
            //     .expect("TODO: panic message");
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
