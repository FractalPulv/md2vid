use std::{error::Error, fs};
use tokio::process::Command;
use tauri::Window;
use regex::Regex;
use std::path::Path;

use crate::yt_downloader;
use crate::log_utils;

pub async fn create_video_with_ffmpeg(
    window: Window,
    frontmatter: &str,
    text_content: &str,
    youtube_url: &str,
    delete_temp_videos: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // println!("===================================");
    // println!("Frontmatter: {}", frontmatter);
    // println!("Text content: {}", text_content);
    // println!("===================================");

    log_utils::print_pretty_log("Removing old audio file...", "blue");
    

    // let old_audio_path = "./temp/audio.mp3";
    // if Path::new(old_audio_path).exists() {
    //     fs::remove_file(old_audio_path)?;
    // }

    log_utils::print_pretty_log("Downloading YouTube video as MP3...", "blue");

    let download_result = yt_downloader::download_youtube_as_mp3(youtube_url);
    match download_result {
        Ok(_) => println!("Video downloaded successfully"),
        Err(e) => {
            println!("Failed to download video: {}", e);
            return Err(e.into());
        }
    }

    log_utils::print_pretty_log("Generating videos for each sentence...", "blue");

    let sentences: Vec<&str> = text_content.split(". ").flat_map(|s| s.split(".\n")).collect();
    let mut file_list = String::new();

    for (i, sentence) in sentences.iter().enumerate() {

        println!("xxxxxxxxxxxxxxxxxxxxxxxx");
        println!("Sentence pre-trim {}", sentence);
        
        // there's a bug with paragraphs having a lot of white space before the sentence
        // for now trim this starting white space
        let sentence = sentence.trim();

        println!("Sentence post-trim {}", sentence);
        println!("xxxxxxxxxxxxxxxxxxxxxxxx");

       // Process the sentence to handle text with and without aliases within double square brackets
       let sentence_with_color = process_sentence(sentence);

        let ass_content = generate_ass_content(&sentence_with_color)?;
        let ass_file_name = format!("sentence{}.ass", i);
        write_ass_file(&ass_file_name, &ass_content)?;

        // Await the output here
        let command_output = execute_ffmpeg_command(&ass_file_name, i).await?;

        // Check the status of the command
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

    concatenate_videos().await?;
    
    // Check if concatenated video exists
    if !fs::metadata("output.mp4").is_ok() {
        return Err("Concatenated video (output.mp4) not found".into());
    }

    // when merge is done send progress
    merge_audio_with_video().await?;

    if delete_temp_videos {
        delete_temporary_videos(&sentences)?;
        delete_file_list()?;
    }

    Ok(())
}

// Function to process sentence with and without aliases within double square brackets
fn process_sentence(sentence: &str) -> String {
    let re = Regex::new(r"\[\[(.*?)\]\]").unwrap();

    let result = re.replace_all(sentence, |caps: &regex::Captures| {
        let text = &caps[1];
        if let Some((_, alias)) = text.split_once(" |") {
            format!("{{\\c&H800080&}}{}{{\\c&HFFFFFF&}}", alias.trim())
        } else {
            format!("{{\\c&H800080&}}{}{{\\c&HFFFFFF&}}", text)
        }
    });

    result.into_owned()
}


fn generate_ass_content(sentence: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let ass_content = format!(
        r#"[Script Info]
        Title: Default Aegisub file
        ScriptType: v4.00+
        WrapStyle: 0
        PlayResX: 1280
        PlayResY: 720
        ScaledBorderAndShadow: yes
        YCbCr Matrix: None
        
        [V4+ Styles]
        Format: Name, Fontname, Fontsize, PrimaryColour, Alignment
        Style: Default, Vera, 42, &HFFFFFF, 8
        
        [Events]
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
        Dialogue: 0,0:00:00.00,0:00:05.00,Default,,320,320,355,,{}"#,
        sentence
    );

    Ok(ass_content)
}

fn write_ass_file(file_name: &str, content: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    fs::write(file_name, content)?;
    Ok(())
}

async fn execute_ffmpeg_command(
    ass_file_name: &str,
    index: usize,
) -> Result<std::process::Output, Box<dyn Error + Send + Sync>> {
    let command_output = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-f",
            "lavfi",
            "-i",
            "color=color=black:size=1280x720",
            "-vf",
            &format!("ass={}:fontsdir=./", ass_file_name),
            "-t",
            "5",
            "-b:v",
            "5M",
            "-preset",
            "slow",
            "-y",
            &format!("output{}.mp4", index),
        ])
        .output()
        .await?;

    Ok(command_output)
}

fn emit_progress_event(
    window: &Window,
    progress: f64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    window.emit("progress", Some(progress))?;
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

async fn concatenate_videos() -> Result<(), Box<dyn Error + Send + Sync>> {
    let command_output = Command::new("ffmpeg")
        .args(&[
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            "file_list.txt",
            "-c",
            "copy",
            "-y",
            "output.mp4",
        ])
        .output()
        .await?;

    if command_output.status.success() {
        println!("Videos concatenated successfully!");
        Ok(())
    } else {
        eprintln!(
            "Error concatenating videos: {}",
            String::from_utf8_lossy(&command_output.stderr)
        );
        Err("Failed to concatenate videos".into())
    }
}

async fn merge_audio_with_video() -> Result<(), Box<dyn Error + Send + Sync>> {
    let command_output = Command::new("ffmpeg")
        .args(&[
            "-y", // Allow overwrite
            "-i",
            "output.mp4",
            "-i",
            "./temp_files/audio.mp3",
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            "-strict",
            "experimental",
            "-map",
            "0:v:0",
            "-map",
            "1:a:0",
            "-shortest",
            "final_output.mp4",
        ])
        .output()
        .await?;

    if command_output.status.success() {
        println!("Audio merged with video successfully!");
        Ok(())
    } else {
        eprintln!(
            "Error merging audio with video: {}",
            String::from_utf8_lossy(&command_output.stderr)
        );
        Err("Failed to merge audio with video".into())
    }
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
