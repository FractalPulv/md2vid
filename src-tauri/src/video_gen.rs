use std::path::Path;
use ndarray::Array3;
use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;
use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};
use tauri::Window;
use tokio::task;
use std::error::Error;
use tokio::process::Command;
use std::fs;

pub async fn create_video_with_ffmpeg(window: Window, paragraph: &str, delete_temp_videos: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Split the paragraph into sentences
    let sentences: Vec<&str> = paragraph.split(". ").collect();

    let mut file_list = String::new();
    // Iterate over each sentence and generate a video
    for (i, sentence) in sentences.iter().enumerate() {
        // Execute FFmpeg command asynchronously
        let command = Command::new("ffmpeg")
            .args(&[
                "-y", // Overwrite output files without asking
                "-f", "lavfi", // Input format
                "-i", "color=color=black:size=1280x720", // Input video with black background
                "-vf", &format!(
                    "drawtext=fontfile=Vera.ttf:fontsize=24:fontcolor=white:text='{}':x=(w-text_w)/2:y=(h-text_h)/2,fade=t=in:st=0:d=1,fade=t=out:st=4:d=1", // Text drawing with fade effects
                    sentence.trim() // Insert sentence text dynamically and trim any leading/trailing whitespace
                ),
                "-t", "5", // Output duration (5 seconds)
                "-b:v", "5M", // Video bitrate
                "-preset", "slow", // Encoding preset for better quality
                "-y", // Overwrite output file without asking
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
    }

    // Write the file list to a text file
    fs::write("file_list.txt", file_list).expect("Unable to write file");

    // Use FFmpeg to concatenate the videos
    let command = Command::new("ffmpeg")
        .args(&[
            "-f", "concat", // Specify the concat demuxer
            "-safe", "0", // Allow unsafe file paths
            "-i", "file_list.txt", // Input file list
            "-c", "copy", // Copy the input streams directly, without re-encoding
            "-y", // Overwrite output file without asking
            "output.mp4" // Output file name
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
