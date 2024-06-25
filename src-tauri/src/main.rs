// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, OnceLock, RwLock};
use rdev::Key;
use std::collections::HashSet;

mod input;

static KEYS: OnceLock<Arc<RwLock<HashSet<rdev::Key>>>> = OnceLock::new();

fn main() {
    tauri::Builder
        ::default()
        .setup(|_app| {
            std::thread::spawn(move || {
                input::start();
            });
            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![keys]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_keys() -> Arc<RwLock<HashSet<Key>>> {
    Arc::clone(KEYS.get_or_init(|| Arc::new(RwLock::new(HashSet::new()))))
}

#[tauri::command]
fn keys() -> Vec<Key> {
    get_keys() //get the arc
        .read().unwrap() // deref the arc and acquire read lock
        .iter() // make an iterator from the set
        .cloned() // clone each element
        .collect::<Vec<_>>()
}
