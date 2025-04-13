#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fontdue::{Font, FontSettings};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::collections::HashSet;

const WIDTH: u32 = 1200; // Screen width
const HEIGHT: u32 = 400; // Screen height

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("KeyLayout").with_inner_size(LogicalSize::new(WIDTH, HEIGHT)).build(&event_loop).unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Failed to create Pixels instance");

    let mut pressed_keys = HashSet::new();
    let mut shift_pressed = false;

    // Load default font
    let font_data = include_bytes!("fonts/OpenSans-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to load font");

    let key_map = vec![
        (VirtualKeyCode::Grave, 50, 50, 50, 50, "`"),
        (VirtualKeyCode::Key1, 110, 50, 50, 50, "1"),
        (VirtualKeyCode::Key2, 170, 50, 50, 50, "2"),
        (VirtualKeyCode::Key3, 230, 50, 50, 50, "3"),
        (VirtualKeyCode::Key4, 290, 50, 50, 50, "4"),
        (VirtualKeyCode::Key5, 350, 50, 50, 50, "5"),
        (VirtualKeyCode::Key6, 410, 50, 50, 50, "6"),
        (VirtualKeyCode::Key7, 470, 50, 50, 50, "7"),
        (VirtualKeyCode::Key8, 530, 50, 50, 50, "8"),
        (VirtualKeyCode::Key9, 590, 50, 50, 50, "9"),
        (VirtualKeyCode::Key0, 650, 50, 50, 50, "0"),
        (VirtualKeyCode::Minus, 710, 50, 50, 50, "-"),
        (VirtualKeyCode::Equals, 770, 50, 50, 50, "="),
        (VirtualKeyCode::Back, 830, 50, 100, 50, "Backspace"),

        (VirtualKeyCode::Tab, 50, 110, 80, 50, "Tab"),
        (VirtualKeyCode::Q, 140, 110, 50, 50, "q"),
        (VirtualKeyCode::W, 200, 110, 50, 50, "w"),
        (VirtualKeyCode::E, 260, 110, 50, 50, "e"),
        (VirtualKeyCode::R, 320, 110, 50, 50, "r"),
        (VirtualKeyCode::T, 380, 110, 50, 50, "t"),
        (VirtualKeyCode::Y, 440, 110, 50, 50, "y"),
        (VirtualKeyCode::U, 500, 110, 50, 50, "u"),
        (VirtualKeyCode::I, 560, 110, 50, 50, "i"),
        (VirtualKeyCode::O, 620, 110, 50, 50, "o"),
        (VirtualKeyCode::P, 680, 110, 50, 50, "p"),
        (VirtualKeyCode::LBracket, 740, 110, 50, 50, "["),
        (VirtualKeyCode::RBracket, 800, 110, 50, 50, "]"),
        (VirtualKeyCode::Backslash, 860, 110, 90, 50, "\\"),

        (VirtualKeyCode::Capital, 50, 170, 90, 50, "Caps"),
        (VirtualKeyCode::A, 150, 170, 50, 50, "a"),
        (VirtualKeyCode::S, 210, 170, 50, 50, "s"),
        (VirtualKeyCode::D, 270, 170, 50, 50, "d"),
        (VirtualKeyCode::F, 330, 170, 50, 50, "f"),
        (VirtualKeyCode::G, 390, 170, 50, 50, "g"),
        (VirtualKeyCode::H, 450, 170, 50, 50, "h"),
        (VirtualKeyCode::J, 510, 170, 50, 50, "j"),
        (VirtualKeyCode::K, 570, 170, 50, 50, "k"),
        (VirtualKeyCode::L, 630, 170, 50, 50, "l"),
        (VirtualKeyCode::Semicolon, 690, 170, 50, 50, ";"),
        (VirtualKeyCode::Apostrophe, 750, 170, 50, 50, "'"),
        (VirtualKeyCode::Return, 810, 170, 130, 50, "Enter"),

        (VirtualKeyCode::LShift, 50, 230, 110, 50, "Shift"),
        (VirtualKeyCode::Z, 170, 230, 50, 50, "z"),
        (VirtualKeyCode::X, 230, 230, 50, 50, "x"),
        (VirtualKeyCode::C, 290, 230, 50, 50, "c"),
        (VirtualKeyCode::V, 350, 230, 50, 50, "v"),
        (VirtualKeyCode::B, 410, 230, 50, 50, "b"),
        (VirtualKeyCode::N, 470, 230, 50, 50, "n"),
        (VirtualKeyCode::M, 530, 230, 50, 50, "m"),
        (VirtualKeyCode::Comma, 590, 230, 50, 50, ","),
        (VirtualKeyCode::Period, 650, 230, 50, 50, "."),
        (VirtualKeyCode::Slash, 710, 230, 50, 50, "/"),
        (VirtualKeyCode::RShift, 770, 230, 170, 50, "Shift"),

        (VirtualKeyCode::LControl, 50, 290, 80, 50, "Ctrl"),
        (VirtualKeyCode::LAlt, 140, 290, 80, 50, "Alt"),
        (VirtualKeyCode::Space, 230, 290, 400, 50, "Space"),
        (VirtualKeyCode::RAlt, 640, 290, 80, 50, "Alt"),
        (VirtualKeyCode::RControl, 730, 290, 80, 50, "Ctrl"),
    ];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                pressed_keys.insert(keycode);

                                // Check if Shift key is pressed
                                if keycode == VirtualKeyCode::LShift || keycode == VirtualKeyCode::RShift {
                                    shift_pressed = true;
                                }
                            }
                            ElementState::Released => {
                                pressed_keys.remove(&keycode);

                                // Check if Shift key is released
                                if keycode == VirtualKeyCode::LShift || keycode == VirtualKeyCode::RShift {
                                    shift_pressed = false;
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();

                // Background
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF]); // Black with full alpha
                }

                // Draw keys and letters
                for &(key, x, y, width, height, label) in &key_map {
                    let color = if pressed_keys.contains(&key) {
                        [255, 255, 255, 255] // Red for pressed keys
                    } else {
                        [125, 125, 125, 255] // Gray for idle keys
                    };
                    draw_rectangle(frame, x, y, width, height, color);

                    // Render the key's label as transparent text
                    let text = if shift_pressed && label.len() == 1 && label.chars().all(|c| c.is_alphanumeric()) {
                        label.to_uppercase()
                    } else {
                        label.to_string()
                    };

                    draw_text_centered(&font, frame, x, y, width, height, &text);
                }

                // Render the frame
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    eprintln!("Pixel rendering error occurred!");
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

/// Draw a rectangle on the frame buffer
fn draw_rectangle(frame: &mut [u8], x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
    for row in 0..height {
        for col in 0..width {
            let idx = ((y + row) * WIDTH + (x + col)) as usize * 4;
            if idx + 3 < frame.len() {
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}

/// Draw text using fontdue and center it within the rectangle
fn draw_text_centered(
    font: &Font,
    frame: &mut [u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    text: &str,
) {
    let mut text_width = 0;
    let mut text_height = 0;

    // Calculate the total width and height of the text
    for c in text.chars() {
        let (metrics, _) = font.rasterize(c, 20.0);
        text_width += metrics.advance_width as u32;
        text_height = text_height.max(metrics.height as u32);
    }

    // Calculate the starting position to center the text
    let x_text = x + (width - text_width) / 2;
    let y_text = y + (height - text_height) / 2;

    // Render each character
    let mut current_x = x_text;
    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, 20.0);
        for (dy, row) in bitmap.chunks(metrics.width).enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                let px = current_x + dx as u32;
                let py = y_text + dy as u32;

                if px < WIDTH && py < HEIGHT {
                    let idx = (py * WIDTH + px) as usize * 4;

                    // Blend the glyph pixel with the background (transparent rendering)
                    let alpha = pixel as f32 / 255.0;
                    frame[idx] = (pixel as f32 * alpha + frame[idx] as f32 * (1.0 - alpha)) as u8; // R
                    frame[idx + 1] = (pixel as f32 * alpha + frame[idx + 1] as f32 * (1.0 - alpha)) as u8; // G
                    frame[idx + 2] = (pixel as f32 * alpha + frame[idx + 2] as f32 * (1.0 - alpha)) as u8; // B
                    frame[idx + 3] = 0xFF; // Fully opaque alpha
                }
            }
        }
        current_x += metrics.advance_width as u32;
    }
}