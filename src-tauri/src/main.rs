// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use tauri::Manager;

// Import the file_utils module
mod file_utils;

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_all_files_frontmatter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_all_files_frontmatter() -> Result<String, String> {
    file_utils::get_all_files_frontmatter()
}
