// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use window_vibrancy::apply_acrylic;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            let _ = apply_acrylic(&window, Some((0, 0, 0, 10)));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
