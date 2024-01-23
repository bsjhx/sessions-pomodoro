// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::__cmd__get_initial_time;
use app::__cmd__start_cycle;
use app::configuration::ApplicationSettings;
use app::work_cycle::application_context::ApplicationContext;
use app::work_cycle::facade::{end_current_session, finish_cycle, get_initial_time, start_cycle};
use app::{__cmd__end_current_session, db};
use app::{__cmd__finish_cycle, configuration};
use core::default::Default;
use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let settings = load_settings();

            let pool = db::init(settings.db_file_path.as_ref());
            let pool = pool.clone();
            let connection = pool.get().unwrap();

            let app_context = ApplicationContext::new(settings.work_cycle_settings, connection);
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

fn load_settings() -> ApplicationSettings {
    let settings_path = get_settings_path();
    let settings_path = settings_path.as_str();
    let settings = configuration::load_settings_from_file(settings_path);
    let settings = settings.unwrap_or_else(|| {
        configuration::save_default_settings_to_file(settings_path).unwrap_or_default()
    });
    settings
}

fn get_settings_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/sessions-pomodoro/settings.json"
}
