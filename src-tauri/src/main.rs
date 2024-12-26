// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use std::path::PathBuf;
use std::sync::{ Arc, OnceLock, RwLock };
use std::collections::HashSet;
use controller::Controller;
use tauri::api::dialog::blocking::FileDialogBuilder;

mod kbm;
mod controller;
mod controllers;

static KEYS: OnceLock<Arc<RwLock<HashSet<String>>>> = OnceLock::new();
static CONTROLLER: OnceLock<Arc<RwLock<Controller>>> = OnceLock::new();
pub static SHOULD_POLL_CONTROLLER: OnceLock<Arc<RwLock<bool>>> = OnceLock::new();

fn main() {
    tauri::Builder
        ::default()
        .setup(|_app| {
            std::thread::spawn(move || {
                kbm::start();
            });
            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                keys,
                get_layout,
                unstick_key,
                controller,
                start_controller_polling,
                stop_controller_polling
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// General
#[tauri::command]
async fn get_layout(previous_path: Option<String>) -> Option<(String, String, String)> {
    let mut json_data: Option<String> = None;
    let mut css_data: Option<String> = None;
    let path: PathBuf;
    if let Some(previous) = previous_path {
        path = PathBuf::from(previous);
    } else {
        path = FileDialogBuilder::new().pick_folder()?;
    }
    let dir = fs::read_dir(path.clone()).ok()?;
    for entry in dir {
        let entry = entry.ok()?;
        if entry.file_name() == "layout.json" {
            json_data = Some(fs::read_to_string(entry.path()).ok()?);
            continue;
        }
        if entry.file_name() == "layout.css" {
            css_data = Some(fs::read_to_string(entry.path()).ok()?);
            continue;
        }
    }

    if let (Some(json), Some(css)) = (json_data, css_data) {
        let path_str = path.to_str()?.to_string();
        return Some((json, css, path_str));
    }
    return None;
}

// KBM
pub fn get_keys() -> Arc<RwLock<HashSet<String>>> {
    Arc::clone(KEYS.get_or_init(|| Arc::new(RwLock::new(HashSet::new()))))
}

#[tauri::command]
fn keys() -> Vec<String> {
    let k = get_keys();
    let keys = k //get the arc
        .read()
        .unwrap() // deref the arc and acquire read lock
        .iter() // make an iterator from the set
        .cloned() // clone each element
        .collect::<Vec<_>>();
    k.write() // Clear any mouse wheel / move events
        .unwrap()
        .retain(|k| !k.starts_with("mw_"));
    keys
}

#[tauri::command]
async fn unstick_key(key: String) {
    let keys = get_keys();
    keys.write().unwrap().remove(&key);
}

// Controller
pub fn get_controller() -> Arc<RwLock<Controller>> {
    Arc::clone(CONTROLLER.get_or_init(|| Arc::new(RwLock::new(Controller::new()))))
}

pub fn get_controller_polling_state() -> Arc<RwLock<bool>> {
    Arc::clone(SHOULD_POLL_CONTROLLER.get_or_init(|| Arc::new(RwLock::new(false))))
}

#[tauri::command]
fn controller() -> Controller {
    let arc_rwlock_clone = Arc::clone(&get_controller());
    let controller = arc_rwlock_clone.read().unwrap();
    controller.clone()
}

#[tauri::command]
fn stop_controller_polling() {
    let state = get_controller_polling_state();
    *state.write().unwrap() = false;
}

#[tauri::command]
fn start_controller_polling() {
    let state = get_controller_polling_state();
    if *state.read().unwrap() == false {
        stop_controller_polling();
    }
    *state.write().unwrap() = true;
    std::thread::spawn(move || {
        controller::start();
    });
}
