#[tauri::command]
fn log_(name: &str) -> String {
    format!("Hello, {}!", name)
}
