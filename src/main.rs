#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use config::Config;
use fontdue::{Font, FontSettings, Metrics};
use rdev::{listen, Event, EventType, Key};
use minifb::{Window, WindowOptions, Key as minifbKey, MouseButton};
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
    thread,
    sync::mpsc,
};

const WIDTH: usize = 890;
const HEIGHT: usize = 290;

fn main() {
    let mut config = Config::load_from_file("KeyLayout.ini");
    
    let frame_time = Duration::from_secs_f32(1.0 / 60.0);
    let mut last_frame = Instant::now();

    let (key_sender, key_receiver) = mpsc::channel();

    thread::spawn(move || {
        listen(move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let _ = key_sender.send((key, true));
                }
                EventType::KeyRelease(key) => {
                    let _ = key_sender.send((key, false));
                }
                _ => {}
            }
        }).expect("Global key listener failed");
    });

    let mut window = Window::new("KeyLayout", WIDTH, HEIGHT, WindowOptions::default()).expect("Failed to create window");

    let mut background_frame = vec![0; WIDTH * HEIGHT];
    let mut buffer = vec![0; WIDTH * HEIGHT];

    let mut active_keys = HashSet::new();

    let font_data = include_bytes!("fonts/OpenSans-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to load font");

    let key_map = vec![
        (Key::Num1,          0,   0,  50,  50,  "1"),
        (Key::Num2,          60,  0,  50,  50,  "2"),
        (Key::Num3,          120, 0,  50,  50,  "3"),
        (Key::Num4,          180, 0,  50,  50,  "4"),
        (Key::Num5,          240, 0,  50,  50,  "5"),
        (Key::Num6,          300, 0,  50,  50,  "6"),
        (Key::Num7,          360, 0,  50,  50,  "7"),
        (Key::Num8,          420, 0,  50,  50,  "8"),
        (Key::Num9,          480, 0,  50,  50,  "9"),
        (Key::Num0,          540, 0,  50,  50,  "0"),
        (Key::Minus,         600, 0,  50,  50,  "-"),
        (Key::Equal,         660, 0,  50,  50,  "="),
        (Key::Backspace,     720, 0, 135,  50,  "Backspace"),

        (Key::Tab,           0,  60,  80,  50,  "Tab"),
        (Key::KeyQ,          90,  60,  50,  50,  "q"),
        (Key::KeyW,          150, 60,  50,  50,  "w"),
        (Key::KeyE,          210, 60,  50,  50,  "e"),
        (Key::KeyR,          270, 60,  50,  50,  "r"),
        (Key::KeyT,          330, 60,  50,  50,  "t"),
        (Key::KeyY,          390, 60,  50,  50,  "y"),
        (Key::KeyU,          450, 60,  50,  50,  "u"),
        (Key::KeyI,          510, 60,  50,  50,  "i"),
        (Key::KeyO,          570, 60,  50,  50,  "o"),
        (Key::KeyP,          630, 60,  50,  50,  "p"),
        (Key::LeftBracket,   690, 60,  50,  50,  "["),
        (Key::RightBracket,  750, 60,  50,  50,  "]"),
        (Key::BackSlash,     810, 60,  80,  50,  "\\"),

        (Key::CapsLock,      0,  120, 90,  50,  "Caps"),
        (Key::KeyA,          100, 120, 50,  50,  "a"),
        (Key::KeyS,          160, 120, 50,  50,  "s"),
        (Key::KeyD,          220, 120, 50,  50,  "d"),
        (Key::KeyF,          280, 120, 50,  50,  "f"),
        (Key::KeyG,          340, 120, 50,  50,  "g"),
        (Key::KeyH,          400, 120, 50,  50,  "h"),
        (Key::KeyJ,          460, 120, 50,  50,  "j"),
        (Key::KeyK,          520, 120, 50,  50,  "k"),
        (Key::KeyL,          580, 120, 50,  50,  "l"),
        (Key::SemiColon,     640, 120, 50,  50,  ";"),
        (Key::Quote,         700, 120, 50,  50,  "'"),
        (Key::Return,        760, 120, 130, 50,  "Enter"),

        (Key::ShiftLeft,     0,  180, 110, 50,  "Shift"),
        (Key::KeyZ,          120, 180, 50,  50,  "z"),
        (Key::KeyX,          180, 180, 50,  50,  "x"),
        (Key::KeyC,          240, 180, 50,  50,  "c"),
        (Key::KeyV,          300, 180, 50,  50,  "v"),
        (Key::KeyB,          360, 180, 50,  50,  "b"),
        (Key::KeyN,          420, 180, 50,  50,  "n"),
        (Key::KeyM,          480, 180, 50,  50,  "m"),
        (Key::Comma,         540, 180, 50,  50,  ","),
        (Key::Dot,           600, 180, 50,  50,  "."),
        (Key::Slash,         660, 180, 50,  50,  "/"),
        (Key::ShiftRight,    720, 180, 170, 50,  "Shift"),

        (Key::ControlLeft,   0,  240, 80,  50,  "Ctrl"),
        (Key::Alt,           90,  240, 80,  50,  "Alt"),
        (Key::Space,         180, 240, 400, 50,  "Space"),
        (Key::AltGr,         590, 240, 80,  50,  "Alt"),
        (Key::ControlRight,  680, 240, 80,  50,  "Ctrl"),
    ];
    
    let mut rectangle_color = config.rectangle_color;
    let mut rgb_component = ' ';
    let mut scroll_toggle: u8 = 0;

    for &(_key, x, y, width, height, _) in &key_map {
        draw_rectangle(&mut background_frame, x, y, width, height, rectangle_color);
    }

    let mut glyph_cache: HashMap<(char, u32), (Metrics, Vec<u8>)> = HashMap::new();

    while window.is_open() && !window.is_key_down(minifbKey::Escape) {
        let now = Instant::now();

        if let Ok((key, is_pressed)) = key_receiver.try_recv() {
            if is_pressed {
                active_keys.insert(key);
            } else {
                active_keys.remove(&key);
            }
        }
        
        if scroll_toggle == 1 {
            window.get_keys_released().iter().for_each(|key|
                match key {
                    minifbKey::R => {
                        rgb_component = 'R';
                        println!("Component selected: Red");
                    },
                    minifbKey::G => {
                        rgb_component = 'G';
                        println!("Component selected: Green");
                    },
                    minifbKey::B => {
                        rgb_component = 'B';
                        println!("Component selected: Blue");
                    },
                    _ => println!("Valid components are: R - Red, G - Green, B - Blue.")
                }
            );
        }

        if now.duration_since(last_frame) >= frame_time {
            let shift_pressed = active_keys.contains(&Key::ShiftLeft) || active_keys.contains(&Key::ShiftRight);

            if window.get_mouse_down(MouseButton::Middle) && scroll_toggle == 0 {
                scroll_toggle = 1;
                println!("Scroll color: ON");
                thread::sleep(Duration::from_millis(125));
            }
            else if window.get_mouse_down(MouseButton::Middle) && scroll_toggle == 1 {
                scroll_toggle = 0;
                println!("Scroll color: OFF");
                thread::sleep(Duration::from_millis(125));
            }

            buffer.copy_from_slice(&background_frame);

            for &(key, x, y, width, height, label) in &key_map {
                if active_keys.contains(&key) {
                    draw_rectangle(&mut buffer, x, y, width, height, 0xFFFFFFFF);
                }

                let text = if shift_pressed && label.len() == 1 && label.chars().all(|c| c.is_alphanumeric()) {
                    label.to_uppercase()
                } else {
                    label.to_string()
                };

                draw_key_text(&font, &mut buffer, x, y, width, height, &text, &mut glyph_cache);
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Failed to update buffer");
            last_frame = Instant::now();
        } else {
            if scroll_toggle == 1 {
                if let Some(mouse_wheel) = window.get_scroll_wheel() {
                    let scroll_amount = mouse_wheel.1 as i32;

                    scroll_color(&mut rectangle_color, scroll_amount, rgb_component);

                    background_frame.fill(0);

                    for &(_key, x, y, width, height, _) in &key_map {
                        draw_rectangle(&mut background_frame, x, y, width, height, rectangle_color);
                    }
                    
                    config.rectangle_color = rectangle_color;
                    config.save_to_file("KeyLayout.ini");
                }
            }
            window.update();
        }
    }
}

fn draw_rectangle(buffer: &mut [u32], x: usize, y: usize, width: usize, height: usize, color: u32) {
    for row in 0..height {
        for col in 0..width {
            let idx = (y + row) * WIDTH + (x + col);
            unsafe { *buffer.get_unchecked_mut(idx) = color; }
        }
    }
}

fn scroll_color(color: &mut u32, scroll_amount: i32, component: char) {
    let mut red = ((*color >> 16) & 0xFF) as i32;
    let mut green = ((*color >> 8) & 0xFF) as i32;
    let mut blue = (*color & 0xFF) as i32;

    match component {
        'R' => red = (red + scroll_amount).clamp(0x0, 0xFF),
        'G' => green = (green + scroll_amount).clamp(0x0, 0xFF),
        'B' => blue = (blue + scroll_amount).clamp(0x0, 0xFF),
        _ => {
            println!("pick a RGB component first !");
            return;
        }
    }

    *color = (0xFF << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32);
    println!("color: {:x}", color);
}

fn draw_key_text(
    font: &Font,
    buffer: &mut [u32],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    text: &str,
    glyph_cache: &mut HashMap<(char, u32), (Metrics, Vec<u8>)>,
) {
    let font_size = 24;
    let total_text_width: f32 = text.chars().map(|c| {
        let key = (c, font_size);
        let (metrics, _) = *glyph_cache.entry(key).or_insert_with(|| font.rasterize(c, font_size as f32));
        metrics.advance_width
    }).sum();

    let start_x = x as f32 + (width as f32 - total_text_width) / 2.0;
    let key_center = y as f32 + height as f32 / 2.0;

    let mut x_cursor = start_x;
    for c in text.chars() {
        let key = (c, font_size);
        let (metrics, bitmap) = glyph_cache.entry(key).or_insert_with(|| font.rasterize(c, font_size as f32));

        let glyph_center = metrics.height as f32 / 2.0;
        let char_y = key_center - glyph_center - metrics.ymin as f32;
        let char_x = x_cursor + metrics.xmin as f32;

        let char_x = char_x.round() as usize;
        let char_y = char_y.round() as usize;

        for (dy, row) in bitmap.chunks(metrics.width).enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                let px = char_x + dx;
                let py = char_y + dy;
                if px < WIDTH && py < HEIGHT {
                    unsafe {
                        let idx = py * WIDTH + px;
                        let alpha = pixel as f32 / 255.0;
                        let dest_color = *buffer.get_unchecked_mut(idx);
                        let dest_r = ((dest_color >> 16) & 0xFF) as f32;
                        let dest_g = ((dest_color >> 8) & 0xFF) as f32;
                        let dest_b = (dest_color & 0xFF) as f32;
                        let blended_r = (200.0 * alpha + dest_r * (1.0 - alpha)) as u32;
                        let blended_g = (200.0 * alpha + dest_g * (1.0 - alpha)) as u32;
                        let blended_b = (200.0 * alpha + dest_b * (1.0 - alpha)) as u32;
                        *buffer.get_unchecked_mut(idx) = (0xFF << 24) | (blended_r << 16) | (blended_g << 8) | blended_b;
                    }
                }
            }
        }

        x_cursor += metrics.advance_width;
    }
}
