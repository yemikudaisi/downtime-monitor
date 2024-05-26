// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use std::hint;

use tauri::Manager;
use window_vibrancy::apply_acrylic;

mod db;
mod types;
mod schedule_manager;
mod services;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = apply_acrylic(&window, Some((0, 0, 0, 10)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
          db::reset_tables,
          db::heartbeat::insert,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}