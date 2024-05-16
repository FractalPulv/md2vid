use std::path::Path;
use ndarray::Array3;
use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;
use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};
use tauri::Window;
use tokio::task;

use tokio::process::Command;

use std::{error::Error, fs};


pub async fn create_video_with_ffmpeg(
    window: Window,
    paragraph: &str,
    delete_temp_videos: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Split the paragraph into sentences
    let sentences: Vec<&str> = paragraph.split(". ").collect();

    let mut file_list = String::new();

    // Iterate over each sentence and generate a video
    for (i, sentence) in sentences.iter().enumerate() {
        // Modify the sentence to include text color if it's wrapped with double square brackets
        let sentence_with_color = if sentence.contains("[[") && sentence.contains("]]") {
            // Extracting text inside double square brackets and wrapping it with ass style
            let colored_sentence = sentence
                .replace("[[", "{\\c&H800080&}") // Purple color
                .replace("]]", "{\\c&HFFFFFF&}"); // Reset color to white after the colored text
            colored_sentence
        } else {
            // If no coloring is needed, use the original sentence
            sentence.to_string()
        };

        // Generate .ass file content for this sentence
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
        Style: Default, Vera, 24, &HFFFFFF, 8
        
        [Events]
        Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
        Dialogue: 0,0:00:00.00,0:00:05.00,Default,,320,320,355,,{}
        "#,
            sentence_with_color
        );
        
        

        // Write .ass file for this sentence
        let ass_file_name = format!("sentence{}.ass", i);
        fs::write(&ass_file_name, ass_content)?;

        // Execute FFmpeg command asynchronously with the .ass file
        let command = Command::new("ffmpeg")
            .args(&[
                "-y", // Overwrite output files without asking
                "-f",
                "lavfi", // Input format
                "-i",
                "color=color=black:size=1280x720", // Input video with black background
                "-vf",
                &format!(
                    "ass={}:fontsdir=./", // Use the generated .ass file
                    &ass_file_name
                ),
                "-t",
                "5",    // Output duration (5 seconds)
                "-b:v",
                "5M",   // Video bitrate
                "-preset",
                "slow", // Encoding preset for better quality
                "-y",   // Overwrite output file without asking
                &format!("output{}.mp4", i), // Output file name with index
            ])
            .output()
            .await
            .expect("Failed to execute FFmpeg command");

        // Check if FFmpeg command execution was successful
        if command.status.success() {
            // Emit progress event
            let progress = (i + 1) as f64 / sentences.len() as f64 * 100.0;
            println!("Emitting progress: {}", progress);
            window.emit("progress", Some(progress)).expect("Failed to emit progress event");
            file_list.push_str(&format!("file 'output{}.mp4'\n", i));
        } else {
            eprintln!("Error: {}", String::from_utf8_lossy(&command.stderr));
        }

        // Delete the .ass file
        fs::remove_file(&ass_file_name).expect("Failed to delete .ass file");
    }

    // Write the file list to a text file
    fs::write("file_list.txt", file_list)?;

    // Use FFmpeg to concatenate the videos
    let command = Command::new("ffmpeg")
        .args(&[
            "-f",
            "concat",       // Specify the concat demuxer
            "-safe",
            "0",            // Allow unsafe file paths
            "-i",
            "file_list.txt",// Input file list
            "-c",
            "copy",         // Copy the input streams directly, without re-encoding
            "-y",           // Overwrite output file without asking
            "output.mp4",   // Output file name
        ])
        .output()
        .await
        .expect("Failed to execute FFmpeg command");

    // Check if FFmpeg command execution was successful
    if !command.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&command.stderr));
    }

    // Delete temporary videos if specified
    if delete_temp_videos {
        for (i, _) in sentences.iter().enumerate() {
            fs::remove_file(format!("output{}.mp4", i)).expect("Failed to delete temporary video");
        }
        fs::remove_file("file_list.txt").expect("Failed to delete file list");
    }

    Ok(())
}
