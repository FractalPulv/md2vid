use std::{error::Error, fs};
use tokio::process::Command;
use tauri::Window;
use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use crate::image_resolution::ImageResolution;




use crate::yt_downloader;
use crate::log_utils;
use crate::text_processing;
use crate::ffmpeg_operations;

pub async fn create_video_with_ffmpeg(
    window: Window,
    frontmatter: &str,
    text_content: &str,
    youtube_url: &str,
    delete_temp_videos: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    log_utils::print_pretty_log("Removing old audio file...", "blue");

    let old_audio_path = "./temp_files/audio.mp3";
    if Path::new(old_audio_path).exists() {
        fs::remove_file(old_audio_path)?;
    }

    log_utils::print_pretty_log("Downloading YouTube video as MP3...", "blue");
    emit_stage_event(&window, "Downloading Audio")?;

    let download_result = yt_downloader::download_youtube_as_mp3(youtube_url);
    match download_result {
        Ok(_) => println!("Video downloaded successfully"),
        Err(e) => {
            println!("Failed to download video: {}", e);
            return Err(e.into());
        }
    }

    log_utils::print_pretty_log("Generating videos for each sentence...", "blue");
    emit_stage_event(&window, "Generating Videos")?;

    let sentences: Vec<&str> = text_content.split(". ")
        .flat_map(|s| s.split(".\n"))
        .flat_map(|s| s.split("? "))
        .flat_map(|s| s.split("!\n"))
        .flat_map(|s| s.split("! "))
        .collect();

    let sentences: Vec<String> = sentences.iter().map(|s| s.replace("\n", " ")).collect();

    let mut file_list = String::new();

    for (i, sentence) in sentences.iter().enumerate() {
        let sentence = sentence.trim();
        let mut image_file_path = None;

        if let Some(image_path_or_url) = check_image_in_text(sentence) {
            if image_path_or_url.starts_with("http") {
                // Hosted image
                let image_path = format!("./temp_files/image{}.png", i);
                download_image(&image_path_or_url, &image_path).await?;
                image_file_path = Some(image_path);
            } else {
                // Local image
                image_file_path = Some(image_path_or_url);
            }
        }

        let sentence_with_color = text_processing::process_sentence(sentence);
                let ass_content = text_processing::generate_ass_content_bottom(&sentence_with_color)?;
        let ass_file_name = format!("sentence{}.ass", i);
        write_ass_file(&ass_file_name, &ass_content)?;

        let command_output = if let Some(image_path) = image_file_path {
            ffmpeg_operations::generate_video_with_text_and_image(&ass_file_name, &image_path, i, ImageResolution::Low).await?
        } else {
            ffmpeg_operations::execute_ffmpeg_command(&ass_file_name, i).await?
        };

        if command_output.status.success() {
            let progress = (i + 1) as f64 / sentences.len() as f64 * 100.0;
            emit_progress_event(&window, progress)?;
            file_list.push_str(&format!("file 'output{}.mp4'\n", i));
        } else {
            eprintln!(
                "Error: {}",
                String::from_utf8_lossy(&command_output.stderr)
            );
        }

        delete_ass_file(&ass_file_name)?;
    }

    log_utils::print_pretty_log("Generate file list...", "blue");

    write_file_list(&file_list)?;

    log_utils::print_pretty_log("Concatenating....", "blue");
    emit_stage_event(&window, "Concatenating videos");

    ffmpeg_operations::concatenate_videos().await?;

    if !fs::metadata("output.mp4").is_ok() {
        return Err("Concatenated video (output.mp4) not found".into());
    }

    emit_stage_event(&window, "Merging audio");
    ffmpeg_operations::merge_audio_with_video().await?;

    if delete_temp_videos {
        delete_temporary_videos(&sentences.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;
        delete_file_list()?;
    }

    emit_stage_event(&window, "Done");

    Ok(())
}



// 


fn write_ass_file(file_name: &str, content: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    fs::write(file_name, content)?;
    Ok(())
}



fn emit_progress_event(
    window: &Window,
    progress: f64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    window.emit("progress", Some(progress))?;
    Ok(())
}

fn emit_stage_event(
    window: &Window,
    stage: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    window.emit("stage", Some(stage))?;
    Ok(())
}

fn delete_ass_file(file_name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    fs::remove_file(file_name)?;
    Ok(())
}

fn write_file_list(file_list: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    fs::write("file_list.txt", file_list)?;
    Ok(())
}



fn delete_temporary_videos(
    sentences: &[&str],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for (i, _) in sentences.iter().enumerate() {
        fs::remove_file(format!("output{}.mp4", i))?;
    }
    Ok(())
}

fn delete_file_list() -> Result<(), Box<dyn Error + Send + Sync>> {
    fs::remove_file("file_list.txt")?;
    Ok(())
}

fn check_image_in_text(text: &str) -> Option<String> {
    // let local_image_directory = env::var("LOCAL_IMAGE_DIR_PATH").expect("LOCAL_IMAGE_DIR_PATH not found in .env file");
    let local_image_directory = std::env::var("LOCAL_IMAGE_DIR_PATH").expect("LOCAL_IMAGE_DIR_PATH not found in .env file");
    // Local image syntax: ![[image.png]]
    let local_image_regex = Regex::new(r"!\[\[([^\]]+)\]\]").unwrap();
    // Hosted image syntax: ![alt](url.png)
    let hosted_image_regex = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();

    if let Some(captures) = local_image_regex.captures(text) {
        // Extract the image path from the capture group
        let image_path = captures.get(1).unwrap().as_str();
        let full_image_path = Path::new(&local_image_directory).join(image_path);
        Some(full_image_path.to_str().unwrap().to_string())
    } else if let Some(captures) = hosted_image_regex.captures(text) {
        // Extract the image URL from the capture group
        let image_url = captures.get(2).unwrap().as_str().to_string();
        Some(image_url)
    } else {
        None
    }
}


async fn download_image(url: &str, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = Client::new().get(url).send().await?;
    let bytes = response.bytes().await?;
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

