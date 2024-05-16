use std::path::Path;
use ndarray::Array3;
use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;
use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};
use tauri::Window;
use tokio::task;
use std::error::Error;

pub async fn create_video(window: Window) -> Result<(), Box<dyn Error + Send + Sync>> {
    video_rs::init().unwrap(); // Initialize video-rs

    let settings = Settings::preset_h264_yuv420p(1280, 720, false);
    let encoder_path = Path::new("video.mp4");
    let title = "WK lannie"; // Define your title here

    // Spawn a blocking task to run the video encoding loop
    task::spawn_blocking(move || {
        let mut encoder = Encoder::new(encoder_path, settings)?;

        let duration: Time = Time::from_nth_of_a_second(24);
        let mut position = Time::zero();

        for i in 0..256 {
            let frame = black_frame(i as f32 / 256.0, title)?; // Pass the title to the function
        
            encoder.encode(&frame, position)?;
        
            position = position.aligned_with(duration).add();
        
            // Emit a progress event
            println!("Emitting progress: {}", (i + 1) as f64 / 256.0 * 100.0);
            window.emit("progress", Some((i + 1) as f64 / 256.0 * 100.0))?;
        }

        encoder.finish()?;
        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await??;

    Ok(())
}

fn black_frame(p: f32, title: &str) -> Result<Array3<u8>, Box<dyn Error + Send + Sync>> {
    // Load a font
    let font_data = include_bytes!("../assets/Helvetica.ttf"); // Replace with your font file path
    let font = Font::try_from_bytes(font_data as &[u8]).ok_or("Failed to load font")?;
    let scale = Scale::uniform(48.0); // Adjust the font size as needed

    // Generate the black frame
    let rgb = [0, 0, 0];

    // Create a mutable image buffer for drawing
    let mut frame = ImageBuffer::from_fn(1280, 720, |_, _| Rgb([rgb[0], rgb[1], rgb[2]]));

    // Render text onto the image buffer
    render_text(title, &mut frame, &font, scale)?;

    // Convert the image buffer to ndarray::Array3<u8>
    Array3::from_shape_vec((720, 1280, 3), frame.into_raw()).map_err(|_| "Failed to convert frame".into())
}

fn render_text(text: &str, frame: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, scale: Scale) -> Result<(), Box<dyn Error + Send + Sync>> {
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
                    let color = [255, 255, 255]; // White color for text
                    let current_color = pixel.0;
                    let blended_color = blend_colors(current_color, color, alpha);
                    *pixel = Rgb(blended_color);
                }
            });
        }
    }
    Ok(())
}

fn blend_colors(current_color: [u8; 3], new_color: [u8; 3], alpha: u8) -> [u8; 3] {
    let alpha = alpha as f32 / 255.0;
    let r = (new_color[0] as f32 * alpha + current_color[0] as f32 * (1.0 - alpha)) as u8;
    let g = (new_color[1] as f32 * alpha + current_color[1] as f32 * (1.0 - alpha)) as u8;
    let b = (new_color[2] as f32 * alpha + current_color[2] as f32 * (1.0 - alpha)) as u8;
    [r, g, b]
}
