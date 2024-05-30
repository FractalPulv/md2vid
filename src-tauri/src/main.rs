use dotenv::dotenv;
use std::error::Error; // Import the Error trait
use tauri::Window;
use std::process::Command;

// Import the file_utils and video_gen modules
mod file_utils;
mod log_utils;
mod video_gen;
mod yt_downloader;
mod text_processing;
mod ffmpeg_operations;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let url = "https://www.youtube.com/watch?v=uYzZEW4bCro";
    
    // match yt_downloader::download_youtube_as_mp3(url) {
    //     Ok(output) => {
    //         // Handle successful result
    //         // Do something with the output
    //     }
    //     Err(error) => {
    //         // Handle error
    //         eprintln!("Error: {}", error);
    //     }
    // }

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_all_files_frontmatter, create_video_with_ffmpeg, read_file_and_extract_frontmatter, open_in_obsidian])
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

// pub fn extract_youtube_url_from_text_content(text_content: &str) -> Option<String> {
//     let youtube_url_start = text_content.find("src=\"https://www.youtube.com/embed/").map(|i| i + "src=\"".len()).unwrap_or(0);
//     let youtube_url_end = text_content[youtube_url_start..].find("\"").map(|i| youtube_url_start + i).unwrap_or_else(|| text_content.len());
//     let youtube_url = &text_content[youtube_url_start..youtube_url_end];
//     if youtube_url.is_empty() {
//         None
//     } else {
//         Some(youtube_url.to_string())
//     }
// }

// take path and window

#[tauri::command]
async fn create_video_with_ffmpeg(path: &str, window: Window) -> Result<(), String> {
    // Extract frontmatter and text content from the file
    let frontmatter = file_utils::extract_frontmatter(&path).map_err(|e| e.to_string())?;
    let text_content = file_utils::extract_text_content(&path).map_err(|e| e.to_string())?;
    
    // Attempt to extract YouTube URL from the frontmatter
    let youtube_url_from_frontmatter = file_utils::extract_youtube_url_from_text_content(&frontmatter).ok();
    
    // If no URL is found in the frontmatter, attempt to extract it from the text content
    let youtube_url = youtube_url_from_frontmatter
        .or_else(|| file_utils::extract_youtube_url_from_text_content(&text_content).ok())
        .unwrap_or_else(|| "https://www.youtube.com/watch?v=H0j_xIm4fW0".to_string());
    
    // Create video with the extracted YouTube URL
    video_gen::create_video_with_ffmpeg(window, &frontmatter, &text_content, &youtube_url, true)
        .await
        .map_err(|e| e.to_string())
}

//read_file_and_extract_frontmatter
#[tauri::command]
async fn read_file_and_extract_frontmatter(path: &str) -> Result<String, String> {
    file_utils::read_file_and_extract_frontmatter(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_in_obsidian(vault: &str, filename: &str) -> Result<(), String> {
    open_in_obsidian_impl(vault, filename).map_err(|e| e.to_string())
}

fn open_in_obsidian_impl(vault: &str, filename: &str) -> std::io::Result<()> {
    let encoded_vault = urlencoding::encode(vault);
    let encoded_filename = urlencoding::encode(filename);

    let obsidian_uri = format!("obsidian://open?vault={}&file={}", encoded_vault, encoded_filename);
    println!("Opening Obsidian URI: {}", obsidian_uri);

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &format!("start {}", obsidian_uri)])
            .output()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(obsidian_uri)
            .output()?;
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(obsidian_uri)
            .output()?;
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS"));
    }

    Ok(())
}
