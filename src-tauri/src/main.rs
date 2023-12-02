// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application_context;
mod sessions;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
