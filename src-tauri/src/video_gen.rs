use std::path::Path;

use ndarray::Array3;
use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;

use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};

pub fn create_rainbow_video() -> Result<(), Box<dyn std::error::Error>> {
    video_rs::init().unwrap(); // Initialize video-rs

    let settings = Settings::preset_h264_yuv420p(1280, 720, false);
    let mut encoder = Encoder::new(Path::new("rainbow.mp4"), settings)?;

    let duration: Time = Time::from_nth_of_a_second(24);
    let mut position = Time::zero();
    let title = "Rainbow Title"; // Define your title here

    for i in 0..256 {
        let frame = rainbow_frame(i as f32 / 256.0, title); // Pass the title to the function

        encoder.encode(&frame, position)?;

        position = position.aligned_with(duration).add();
    }

    encoder.finish()?;

    Ok(())
}

fn rainbow_frame(p: f32, title: &str) -> Array3<u8> {
    // Load a font
    let font_data = include_bytes!("../assets/Helvetica.ttf"); // Replace with your font file path
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale::uniform(48.0); // Adjust the font size as needed

    // Generate the rainbow frame
    let rgb = hsv_to_rgb(p * 360.0, 100.0, 100.0);

    // Create a mutable image buffer for drawing
    let mut frame = ImageBuffer::from_fn(1280, 720, |x, y| Rgb([rgb[0], rgb[1], rgb[2]]));

    // Render text onto the image buffer
    render_text(title, &mut frame, &font, scale);

    // Convert the image buffer to ndarray::Array3<u8>
    Array3::from_shape_vec((720, 1280, 3), frame.into_raw()).unwrap_or_else(|_| panic!("Failed to convert frame"))
}

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
                    let color = [alpha, alpha, alpha];
                    let current_color = pixel.0;
                    let blended_color = blend_colors(current_color, color);
                    *pixel = Rgb(blended_color);
                }
            });
        }
    }
}


fn blend_colors(current_color: [u8; 3], new_color: [u8; 3]) -> [u8; 3] {
    let alpha = new_color[0] as f32 / 255.0;
    let r = (new_color[0] as f32 * alpha + current_color[0] as f32 * (1.0 - alpha)) as u8;
    let g = (new_color[1] as f32 * alpha + current_color[1] as f32 * (1.0 - alpha)) as u8;
    let b = (new_color[2] as f32 * alpha + current_color[2] as f32 * (1.0 - alpha)) as u8;
    [r, g, b]
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let s = s / 100.0;
    let v = v / 100.0;
    let c = s * v;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;
    let (r, g, b) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, c)
    } else if (300.0..360.0).contains(&h) {
        (c, 0.0, x)
    } else {
        (0.0, 0.0, 0.0)
    };
    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}
