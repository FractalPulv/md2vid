use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;

use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};

use ndarray::Array3;
use std::error::Error;
use std::path::Path;

use crate::yt_downloader;

// Define a function to create a video with black background and white text using the downloaded MP3 as audio
pub async fn create_black_video_with_audio() -> Result<(), Box<dyn Error>> {
    // Download the MP3
    let youtube_url = "https://www.youtube.com/watch?v=uYzZEW4bCro";
    let mp3_file_path = std::env::current_dir().unwrap().join("output").join("audio.mp3");

    yt_downloader::download_youtube_as_mp3(youtube_url);

    // Check if the download was successful
    if mp3_file_path.exists() {
        // Rename the downloaded file
        let mp3_file_path_clone = mp3_file_path.clone();
        std::fs::rename(mp3_file_path, &mp3_file_path_clone)?;
    } else {
        return Err("Failed to download the MP3 file".into());
    }

    // Initialize video-rs
    video_rs::init().unwrap();

    // Define video settings
    let settings = Settings::preset_h264_yuv420p(1280, 720, false);
    let mut encoder = Encoder::new(Path::new("black_with_audio.mp4"), settings)?;

    // Set video duration and initial position
    let duration: Time = Time::from_nth_of_a_second(24);
    let mut position = Time::zero();
    let title = "Black Title"; // Define your title here

    // Iterate through frames
    for i in 0..256 {
        // Create black frame with white text
        let frame = black_frame(i as f32 / 256.0, title); // Pass the title to the function

        // Encode frame
        encoder.encode(&frame, position)?;

        // Increment position
        position = position.aligned_with(duration).add();
    }

    // Finish encoding
    encoder.finish()?;

    Ok(())
}

// Define function to create black frame with white text
fn black_frame(p: f32, title: &str) -> Array3<u8> {
    // Load a font
    let font_data = include_bytes!("../assets/Helvetica.ttf"); // Replace with your font file path
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale::uniform(48.0); // Adjust the font size as needed

    // Generate the black frame
    let rgb = [0, 0, 0];

    // Create a mutable image buffer for drawing
    let mut frame = ImageBuffer::from_fn(1280, 720, |x, y| Rgb([rgb[0], rgb[1], rgb[2]]));

    // Render text onto the image buffer
    render_text(title, &mut frame, &font, scale);

    // Convert the image buffer to ndarray::Array3<u8>
    Array3::from_shape_vec((720, 1280, 3), frame.into_raw()).unwrap_or_else(|_| panic!("Failed to convert frame"))
}

// Define function to render white text
fn render_text(text: &str, frame: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, scale: Scale) {
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(0.0, v_metrics.ascent)).collect();

    let frame_width = frame.width() as i32;
    let frame_height = frame.height() as i32;

    let text_width: i32 = glyphs.iter().map(|g| g.unpositioned().h_metrics().advance_width as i32).sum();
    let text_height: i32 = (v_metrics.ascent - v_metrics.descent) as i32; // Convert to i32

    let x_offset = (frame_width - text_width) / 2;
    let y_offset = (frame_height - text_height) / 2;

    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x + x_offset;
                let y = y as i32 + bounding_box.min.y + y_offset;
                if x >= 0 && x < frame_width && y >= 0 && y < frame_height {
                    let pixel = frame.get_pixel_mut(x as u32, y as u32);
                    let alpha = (v * 255.0) as u8;
                    let color = [255 - alpha, 255 - alpha, 255 - alpha]; // White text
                    let current_color = pixel.0;
                    let blended_color = blend_colors(current_color, color);
                    *pixel = Rgb(blended_color);
                }
            });
        }
    }
}

// Function to blend colors
fn blend_colors(current_color: [u8; 3], new_color: [u8; 3]) -> [u8; 3] {
    let alpha = new_color[0] as f32 / 255.0;
    let r = (new_color[0] as f32 * alpha + current_color[0] as f32 * (1.0 - alpha)) as u8;
    let g = (new_color[1] as f32 * alpha + current_color[1] as f32 * (1.0 - alpha)) as u8;
    let b = (new_color[2] as f32 * alpha + current_color[2] as f32 * (1.0 - alpha)) as u8;
    [r, g, b]
}