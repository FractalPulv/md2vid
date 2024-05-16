// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::error::Error; // Import the Error trait
use tauri::Window;

// Import the file_utils and video_gen modules
mod file_utils;
mod video_gen;
mod yt_downloader;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = "https://www.youtube.com/watch?v=uYzZEW4bCro";
    

    match yt_downloader::download_youtube_as_mp3(url) {
        Ok(output) => {
            // Handle successful result
            // Do something with the output
        }
        Err(error) => {
            // Handle error
            eprintln!("Error: {}", error);
        }
    }

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_all_files_frontmatter, create_video_with_ffmpeg, read_file_and_extract_frontmatter])
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

// #[tauri::command]
// fn create_rainbow_video() -> Result<(), String> {
//     video_gen::create_rainbow_video().map_err(|e| e.to_string())
// }

// take path and window
#[tauri::command]
async fn create_video_with_ffmpeg(path: &str, window: Window) -> Result<(), String> {
    let frontmatter = file_utils::extract_frontmatter(&path).map_err(|e| e.to_string())?;
    let text_content = file_utils::extract_text_content(&path).map_err(|e| e.to_string())?;
    video_gen::create_video_with_ffmpeg(window, &frontmatter, &text_content, true).await.map_err(|e| e.to_string())
}

//read_file_and_extract_frontmatter
#[tauri::command]
async fn read_file_and_extract_frontmatter(path: &str) -> Result<String, String> {
    file_utils::read_file_and_extract_frontmatter(path).map_err(|e| e.to_string())
}