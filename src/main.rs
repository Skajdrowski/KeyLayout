#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fontdue::{Font, FontSettings};
use minifb::{Key, Window, WindowOptions};
use std::{time::{Duration, Instant}, thread::sleep};

const WIDTH: usize = 1200; // Screen width
const HEIGHT: usize = 400; // Screen height

fn main() {
    let frame_time = Duration::from_secs_f32(1.0 / 60.0); // 60 FPS
    let mut last_frame = Instant::now();

    let mut window = Window::new(
        "KeyLayout",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut shift_pressed = false;

    // Load default font
    let font_data = include_bytes!("fonts/OpenSans-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to load font");

    let key_map = vec![
        (Key::Key1,        50,  50,  50,  50,  "1"),
        (Key::Key2,       110,  50,  50,  50,  "2"),
        (Key::Key3,       170,  50,  50,  50,  "3"),
        (Key::Key4,       230,  50,  50,  50,  "4"),
        (Key::Key5,       290,  50,  50,  50,  "5"),
        (Key::Key6,       350,  50,  50,  50,  "6"),
        (Key::Key7,       410,  50,  50,  50,  "7"),
        (Key::Key8,       470,  50,  50,  50,  "8"),
        (Key::Key9,       530,  50,  50,  50,  "9"),
        (Key::Key0,       590,  50,  50,  50,  "0"),
        (Key::Minus,      650,  50,  50,  50,  "-"),
        (Key::Equal,      710,  50,  50,  50,  "="),
        (Key::Backspace,  770,  50, 110,  50,  "Backspace"),

        (Key::Tab,         50, 110,  80,  50,  "Tab"),
        (Key::Q,          140, 110,  50,  50,  "q"),
        (Key::W,          200, 110,  50,  50,  "w"),
        (Key::E,          260, 110,  50,  50,  "e"),
        (Key::R,          320, 110,  50,  50,  "r"),
        (Key::T,          380, 110,  50,  50,  "t"),
        (Key::Y,          440, 110,  50,  50,  "y"),
        (Key::U,          500, 110,  50,  50,  "u"),
        (Key::I,          560, 110,  50,  50,  "i"),
        (Key::O,          620, 110,  50,  50,  "o"),
        (Key::P,          680, 110,  50,  50,  "p"),
        (Key::LeftBracket,740, 110,  50,  50,  "["),
        (Key::RightBracket,800, 110,  50,  50,  "]"),
        (Key::Backslash,  860, 110,  90,  50,  "\\"),

        (Key::CapsLock,    50, 170,  90,  50,  "Caps"),
        (Key::A,          150, 170,  50,  50,  "a"),
        (Key::S,          210, 170,  50,  50,  "s"),
        (Key::D,          270, 170,  50,  50,  "d"),
        (Key::F,          330, 170,  50,  50,  "f"),
        (Key::G,          390, 170,  50,  50,  "g"),
        (Key::H,          450, 170,  50,  50,  "h"),
        (Key::J,          510, 170,  50,  50,  "j"),
        (Key::K,          570, 170,  50,  50,  "k"),
        (Key::L,          630, 170,  50,  50,  "l"),
        (Key::Semicolon,  690, 170,  50,  50,  ";"),
        (Key::Apostrophe, 750, 170,  50,  50,  "'"),
        (Key::Enter,      810, 170, 130,  50,  "Enter"),

        (Key::LeftShift,   50, 230, 110,  50,  "Shift"),
        (Key::Z,          170, 230,  50,  50,  "z"),
        (Key::X,          230, 230,  50,  50,  "x"),
        (Key::C,          290, 230,  50,  50,  "c"),
        (Key::V,          350, 230,  50,  50,  "v"),
        (Key::B,          410, 230,  50,  50,  "b"),
        (Key::N,          470, 230,  50,  50,  "n"),
        (Key::M,          530, 230,  50,  50,  "m"),
        (Key::Comma,      590, 230,  50,  50,  ","),
        (Key::Period,     650, 230,  50,  50,  "."),
        (Key::Slash,      710, 230,  50,  50,  "/"),
        (Key::RightShift, 770, 230, 170,  50,  "Shift"),

        (Key::LeftCtrl,    50, 290,  80,  50,  "Ctrl"),
        (Key::LeftAlt,    140, 290,  80,  50,  "Alt"),
        (Key::Space,      230, 290, 400,  50,  "Space"),
        (Key::RightAlt,   640, 290,  80,  50,  "Alt"),
        (Key::RightCtrl,  730, 290,  80,  50,  "Ctrl"),
    ];

    let mut background_frame = vec![0; WIDTH * HEIGHT];
    for &(_key, x, y, width, height, _) in &key_map {
        draw_rectangle(&mut background_frame, x, y, width, height, 0x7D7D7DFF); // Gray color
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Frame timing
        let now = Instant::now();
        if now.duration_since(last_frame) >= frame_time {
            last_frame = now;

            // Clear the buffer
            buffer.copy_from_slice(&background_frame);

            // Draw pressed keys
            for &(key, x, y, width, height, label) in &key_map {
                if window.is_key_down(key) {
                    draw_rectangle(&mut buffer, x, y, width, height, 0xFFFFFFFF); // White color
                }
                let text = if shift_pressed && label.len() == 1 && label.chars().all(|c| c.is_alphanumeric()) {
                    label.to_uppercase()
                } else {
                    label.to_string()
                };
                draw_text_centered(&font, &mut buffer, x, y, width, height, &text);
            }

            // Update the window
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .expect("Failed to update buffer");
        }

        // Handle shift pressed state
        shift_pressed = window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift);
        sleep(frame_time.saturating_sub(now.duration_since(last_frame)));
    }
}

/// Draw a rectangle on the frame buffer
fn draw_rectangle(buffer: &mut [u32], x: usize, y: usize, width: usize, height: usize, color: u32) {
    for row in 0..height {
        for col in 0..width {
            let idx = (y + row) * WIDTH + (x + col);
            if idx < buffer.len() {
                buffer[idx] = color;
            }
        }
    }
}

/// Draw text using fontdue and center it within the rectangle
fn draw_text_centered(
    font: &Font,
    buffer: &mut [u32],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    text: &str,
) {
    let mut total_text_width = 0;
    let mut max_text_height = 0;

    // Calculate total text width and maximum height for all characters in the string
    for c in text.chars() {
        let (metrics, _) = font.rasterize(c, 20.0); // Fixed font size
        total_text_width += metrics.advance_width as usize;
        max_text_height = max_text_height.max(metrics.height as usize);
    }

    // Calculate the top-left corner for the text to center it in the rectangle
    let mut x_start = x + (width.saturating_sub(total_text_width)) / 2;
    let mut y_start = y + (height.saturating_sub(max_text_height)) / 2;

    for c in text.chars() {
        // Rasterize each character
        let (metrics, bitmap) = font.rasterize(c, 20.0);

        // Draw the character onto the buffer
        for (dy, row) in bitmap.chunks(metrics.width).enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                let px = x_start + dx;
                let py = y_start + dy;

                if px < WIDTH && py < HEIGHT {
                    let idx = py * WIDTH + px;
                    let alpha = pixel as f32 / 255.0;

                    // Extract the destination color (background)
                    let dest_color = buffer[idx];
                    let dest_r = ((dest_color >> 16) & 0xFF) as f32;
                    let dest_g = ((dest_color >> 8) & 0xFF) as f32;
                    let dest_b = (dest_color & 0xFF) as f32;

                    // Blend the source color (white text) with the background
                    let blended_r = (255.0 * alpha + dest_r * (1.0 - alpha)) as u32;
                    let blended_g = (255.0 * alpha + dest_g * (1.0 - alpha)) as u32;
                    let blended_b = (255.0 * alpha + dest_b * (1.0 - alpha)) as u32;

                    buffer[idx] = (0xFF << 24) | (blended_r << 16) | (blended_g << 8) | blended_b;
                }
            }
        }

        // Advance the current position by the character's advance
        x_start += metrics.advance_width as usize;
        y_start += metrics.advance_height as usize;
    }
}