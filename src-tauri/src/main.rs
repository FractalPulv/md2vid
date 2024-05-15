// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;

// Import the file_utils and video_gen modules
mod file_utils;
mod video_gen;

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_all_files_frontmatter, create_rainbow_video])
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

#[tauri::command]
fn create_rainbow_video() -> Result<(), String> {
    video_gen::create_rainbow_video().map_err(|e| e.to_string())
}
