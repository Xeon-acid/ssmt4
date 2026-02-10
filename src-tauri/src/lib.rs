pub mod configs;
pub mod utils;

use configs::app_config::AppConfig;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_settings(config: AppConfig) -> Result<(), String> {
    config.save()
}

#[tauri::command]
fn load_settings() -> AppConfig {
    AppConfig::load()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, save_settings, load_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
