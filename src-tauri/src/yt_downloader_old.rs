use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use youtube_dl::{YoutubeDl, YoutubeDlOutput, Error};

pub async fn download_youtube_as_mp3(url: &str) -> io::Result<()> {
    // Initialize YoutubeDl with options to download as mp3
    let mut ydl = YoutubeDl::new(url);

    // Set the audio format to download. One of best, aac, vorbis, opus, or mp3.
    ydl.format("ba");

    // Specify the filename template. Only relevant for downloading.
    // Actually make the name of the file 'audio.mp3'
    ydl.output_template("audio.mp3");

    // Set the --extract-audio command line flag.
    ydl.extract_audio(true);

    // Set the output directory for the downloaded file.
    ydl.output_directory("../output");

    // Run yt-dlp with the arguments specified through the builder.
    if let Err(e) = ydl.download_to_async("../output").await {
        eprintln!("Error downloading: {:?}", e);
        return Ok(());
    }

    // Run yt-dlp asynchronously with the arguments specified through the builder.
    let result: Result<YoutubeDlOutput, Box<dyn std::error::Error>> = match ydl.run_async().await {
        Ok(output) => Ok(output),
        Err(e) => {
            eprintln!("Error running yt-dlp: {:?}", e);
            return Ok(());
        }
    };    

    // Handle the result
    match result {
        Ok(_) => {
            // Check if the file exists after download completes
            let mp3_file_path = PathBuf::from("../output/audio.mp3");
            if mp3_file_path.exists() {
                println!("Download successful: {:?}", mp3_file_path);
            } else {
                println!("Failed to download the MP3 file");
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}


// pub async fn download_youtube_audio(url: &str, output_dir: &Path) -> Result<(), Error> {
//     // Create a new YoutubeDl builder
//     let mut youtube_dl = YoutubeDl::new(url);

//     // Set the path to the youtube-dl executable
//     // get the path from the environment variable YT_DLP_EXECUTABLE_PATH 
//     let yt_dlp_executable_path = std::env::var("YT_DLP_EXECUTABLE_PATH").expect("YT_DLP_EXECUTABLE_PATH not found in .env file");
//     println!("YT_DLP_EXECUTABLE_PATH: {:?}", yt_dlp_executable_path);
//     youtube_dl.youtube_dl_path(yt_dlp_executable_path);

//     let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
//     .socket_timeout("15")
//     .run_async()
//     .await?;

//     let title = output.into_single_video().unwrap().title;
//     println!("Video title: {}", title);
    
//     // return Ok(());


// }

// pub fn get_video_title(url: &str) -> Result<String, Error> {
//     let output = YoutubeDl::new(url)
//         .socket_timeout("15")
//         .run()
//         .map_err(Error::from)?;
//     let title = output.into_single_video().ok_or(Error::from(Error::NoVideoFound))?.title;
//     Ok(title)
// }

// pub async fn download_youtube_audio(url: &str, output_dir: &Path) -> Result<(), Error> {
//     // Create a new YoutubeDl builder
//     let mut youtube_dl = YoutubeDl::new(url);

//     // Set the path to the youtube-dl executable
//     // get the path from the environment variable YT_DLP_EXECUTABLE_PATH 
//     let yt_dlp_executable_path = std::env::var("YT_DLP_EXECUTABLE_PATH").expect("YT_DLP_EXECUTABLE_PATH not found in .env file");
//     println!("YT_DLP_EXECUTABLE_PATH: {:?}", yt_dlp_executable_path);
//     youtube_dl.youtube_dl_path(yt_dlp_executable_path);

//     // Set the format to audio only
//     youtube_dl.format("bestaudio");

//     // Specify the output directory
//     let output_dir_string = output_dir.to_string_lossy().to_string();
//     youtube_dl.output_directory(output_dir_string);

//     // Download the audio
//     match youtube_dl.download_to(output_dir) {
//         Ok(_) => {
//             println!("Audio download successful");
//             Ok(())
//         },
//         Err(e) => {
//             eprintln!("Error downloading audio: {:?}", e);
//             Err(e)
//         },
//     }
// }