// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::__cmd__get_current_state;
use app::__cmd__get_initial_time;
use app::__cmd__get_states_for_day;
use app::__cmd__start_cycle;
use app::env_variables::{
    read_boolean_variable, POMODORO_DEVTOOLS_ENABLED, POMODORO_ENABLE_TEST_DATA,
};
use app::history::get_states_for_day;
use app::history::HistoryContext;
use app::settings::ApplicationSettings;
use app::work_cycle::work_cycle_context::WorkCycleContext;
use app::work_cycle::{
    end_current_session, finish_cycle, get_current_state, get_initial_time, start_cycle,
};
use app::{__cmd__end_current_session, db};
use app::{__cmd__finish_cycle, settings};
use core::default::Default;
use std::env;
use std::sync::Mutex;
use tauri::Manager;

#[cfg(not(tarpaulin_include))]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let settings = load_settings();

            let enabled_test_data = read_boolean_variable(POMODORO_ENABLE_TEST_DATA);
            let pool = if !enabled_test_data {
                db::init(settings.db_file_path.as_ref())
            } else {
                db::init_with_mock_data("./sample_database.sqlite")
            };

            let pool = pool.clone();
            let connection = pool.get().unwrap();

            let app_context = WorkCycleContext::new(settings.work_cycle_settings, connection);
            app.manage(Mutex::new(app_context));

            let connection = pool.get().unwrap();
            let history_context = HistoryContext::new(connection);
            app.manage(Mutex::new(history_context));

            let enabled_devtools = read_boolean_variable(POMODORO_DEVTOOLS_ENABLED);
            if enabled_devtools {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_cycle,
            finish_cycle,
            end_current_session,
            get_initial_time,
            get_states_for_day,
            get_current_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_settings() -> ApplicationSettings {
    let settings_path = get_settings_path();
    let settings_path = settings_path.as_str();
    let settings = settings::load_settings_from_file(settings_path);
    settings.unwrap_or_else(|| {
        settings::save_default_settings_to_file(settings_path).unwrap_or_default()
    })
}

fn get_settings_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/sessions-pomodoro/settings.json"
}
