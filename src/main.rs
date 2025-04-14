#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fontdue::{Font, FontSettings};
use rdev::{listen, Event, EventType, Key};
use minifb::{Window, WindowOptions, Key as minifbKey};
use std::{collections::HashSet, time::{Duration, Instant}, thread::{self, sleep}, sync::mpsc};

const WIDTH: usize = 1200;
const HEIGHT: usize = 400;

fn main() {
    let frame_time = Duration::from_secs_f32(1.0 / 60.0);
    let mut last_frame = Instant::now();

    let (key_sender, key_receiver) = mpsc::channel();

    thread::spawn(move || {
        listen(move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    if key_sender.send((key, true)).is_err() {
                        eprintln!("Main thread disconnected, stopping key listener.");
                    }
                }
                EventType::KeyRelease(key) => {
                    if key_sender.send((key, false)).is_err() {
                        eprintln!("Main thread disconnected, stopping key listener.");
                    }
                }
                _ => {}
            }
        })
        .expect("Global key listener failed");
    });

    let mut window = Window::new(
        "KeyLayout",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut active_keys = HashSet::new();

    let mut shift_pressed = false;

    // Default font
    let font_data = include_bytes!("fonts/OpenSans-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to load font");

    let key_map = vec![
        (Key::Num1,          50,  50,  50,  50,  "1"),
        (Key::Num2,          110,  50,  50,  50,  "2"),
        (Key::Num3,          170,  50,  50,  50,  "3"),
        (Key::Num4,          230,  50,  50,  50,  "4"),
        (Key::Num5,          290,  50,  50,  50,  "5"),
        (Key::Num6,          350,  50,  50,  50,  "6"),
        (Key::Num7,          410,  50,  50,  50,  "7"),
        (Key::Num8,          470,  50,  50,  50,  "8"),
        (Key::Num9,          530,  50,  50,  50,  "9"),
        (Key::Num0,          590,  50,  50,  50,  "0"),
        (Key::Minus,         650,  50,  50,  50,  "-"),
        (Key::Equal,         710,  50,  50,  50,  "="),
        (Key::Backspace,     770,  50, 110,  50,  "Backspace"),

        (Key::Tab,           50, 110,  80,  50,  "Tab"),
        (Key::KeyQ,          140, 110,  50,  50,  "q"),
        (Key::KeyW,          200, 110,  50,  50,  "w"),
        (Key::KeyE,          260, 110,  50,  50,  "e"),
        (Key::KeyR,          320, 110,  50,  50,  "r"),
        (Key::KeyT,          380, 110,  50,  50,  "t"),
        (Key::KeyY,          440, 110,  50,  50,  "y"),
        (Key::KeyU,          500, 110,  50,  50,  "u"),
        (Key::KeyI,          560, 110,  50,  50,  "i"),
        (Key::KeyO,          620, 110,  50,  50,  "o"),
        (Key::KeyP,          680, 110,  50,  50,  "p"),
        (Key::LeftBracket,   740, 110,  50,  50,  "["),
        (Key::RightBracket,  800, 110,  50,  50,  "]"),
        (Key::BackSlash,     860, 110,  90,  50,  "\\"),

        (Key::CapsLock,      50, 170,  90,  50,  "Caps"),
        (Key::KeyA,          150, 170,  50,  50,  "a"),
        (Key::KeyS,          210, 170,  50,  50,  "s"),
        (Key::KeyD,          270, 170,  50,  50,  "d"),
        (Key::KeyF,          330, 170,  50,  50,  "f"),
        (Key::KeyG,          390, 170,  50,  50,  "g"),
        (Key::KeyH,          450, 170,  50,  50,  "h"),
        (Key::KeyJ,          510, 170,  50,  50,  "j"),
        (Key::KeyK,          570, 170,  50,  50,  "k"),
        (Key::KeyL,          630, 170,  50,  50,  "l"),
        (Key::SemiColon,     690, 170,  50,  50,  ";"),
        (Key::Quote,         750, 170,  50,  50,  "'"),
        (Key::Return,        810, 170, 130,  50,  "Enter"),

        (Key::ShiftLeft,     50, 230, 110,  50,  "Shift"),
        (Key::KeyZ,          170, 230,  50,  50,  "z"),
        (Key::KeyX,          230, 230,  50,  50,  "x"),
        (Key::KeyC,          290, 230,  50,  50,  "c"),
        (Key::KeyV,          350, 230,  50,  50,  "v"),
        (Key::KeyB,          410, 230,  50,  50,  "b"),
        (Key::KeyN,          470, 230,  50,  50,  "n"),
        (Key::KeyM,          530, 230,  50,  50,  "m"),
        (Key::Comma,         590, 230,  50,  50,  ","),
        (Key::Dot,           650, 230,  50,  50,  "."),
        (Key::Slash,         710, 230,  50,  50,  "/"),
        (Key::ShiftRight,    770, 230, 170,  50,  "Shift"),

        (Key::ControlLeft,   50, 290,  80,  50,  "Ctrl"),
        (Key::Alt,           140, 290,  80,  50,  "Alt"),
        (Key::Space,         230, 290, 400,  50,  "Space"),
        (Key::AltGr,         640, 290,  80,  50,  "Alt"),
        (Key::ControlRight,  730, 290,  80,  50,  "Ctrl"),
    ];

    let mut background_frame = vec![0; WIDTH * HEIGHT];
    for &(_key, x, y, width, height, _) in &key_map {
        draw_rectangle(&mut background_frame, x, y, width, height, 0x7D7D7DFF); // Gray color
    }

    while window.is_open() && !window.is_key_down(minifbKey::Escape) {
        // Frame timing
        let now = Instant::now();

        if now.duration_since(last_frame) >= frame_time {
            last_frame = now;

            while let Ok((key, is_pressed)) = key_receiver.try_recv() {
                if is_pressed {
                    active_keys.insert(key);
                } else {
                    active_keys.remove(&key);
                }
            }

            // Clear the buffer
            buffer.copy_from_slice(&background_frame);

            // Draw pressed keys
            for &(key, x, y, width, height, label) in &key_map {
                if active_keys.contains(&key) {
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
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Failed to update buffer");
        }

        // Handle shift pressed state
        shift_pressed = active_keys.contains(&Key::ShiftLeft) || active_keys.contains(&Key::ShiftRight);
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
        max_text_height = max_text_height.max(metrics.height);
    }

    // Calculate the top-left corner for the text to center it in the rectangle
    let mut x_start = x + (width.saturating_sub(total_text_width)) / 2;
    let y_start = y + (height.saturating_sub(max_text_height)) / 2;

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
    }
}