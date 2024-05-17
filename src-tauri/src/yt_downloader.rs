use std::process::{Command, Output};

pub fn download_youtube_as_mp3(url: &str) -> Result<Output, std::io::Error> {
    let output_dir = "./temp_files/audio.mp3"; // Assuming you want the output in the ../output directory

    println!("Downloading YouTube video as MP3...");
    println!("URL: {}", url);

    let output = Command::new("yt-dlp")
        .args(&[
            "--extract-audio",
            "--audio-format", "mp3",
            "-f", "bestaudio",
            "-o", output_dir,
            url,
        ])
        .output();

    println!("Command done!");

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