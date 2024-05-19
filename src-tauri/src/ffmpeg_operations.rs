use std::error::Error;
use tokio::process::Command;

pub async fn execute_ffmpeg_command(
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

pub async fn concatenate_videos() -> Result<(), Box<dyn Error + Send + Sync>> {
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

pub async fn merge_audio_with_video() -> Result<(), Box<dyn Error + Send + Sync>> {
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

// make a function which 