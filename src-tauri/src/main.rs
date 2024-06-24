// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod input;

fn main() {
    tauri::Builder
        ::default()
        .setup(|_app| {
            std::thread::spawn(move || {
                input::start();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
