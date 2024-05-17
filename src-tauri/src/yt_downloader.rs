use std::process::{Command, Output};

pub fn download_youtube_as_mp3(url: &str) -> Result<Output, std::io::Error> {
    let output_dir = "./temp/audio.mp3"; // Change this to a directory not watched by the build system

    // delete ./temp/audio.mp3 if it already exists
    if std::path::Path::new(output_dir).exists() {
        std::fs::remove_file(output_dir)?;
    }

    println!("Downloading YouTube video as MP3...");
    println!("URL: {}", url);

    let command = format!("yt-dlp --extract-audio --audio-format mp3 -f bestaudio -o {} {}", output_dir, url);
    println!("Command: {}", command);

    let output = Command::new("yt-dlp")
        .args(&[
            "--extract-audio",
            "--audio-format", "mp3",
            "-f", "bestaudio",
            "-o", output_dir,
            url,
        ])
        .output();

    // Check if the command was successful
    match &output {
        Ok(output) => {
            if output.status.success() {
                let full_path = std::fs::canonicalize(output_dir)?;
                println!("Download successful!");
                println!("Output path: {:?}", full_path);
            } else {
                eprintln!("Download failed: {:?}", output.status);
            }
        }
        Err(e) => {
            eprintln!("Error running command: {:?}", e);
        }
    }

    output
}