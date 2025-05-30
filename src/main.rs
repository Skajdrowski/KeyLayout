#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod draw;

use config::Config;
use draw::{rectangle, key_text, settings_text};

use fontdue::{Font, FontSettings, Metrics};
use rdev::{listen, Event, EventType, Key};
use minifb::{Window, WindowOptions, Key as minifbKey, MouseButton};
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
    thread,
    sync::mpsc,
};

const WINDOW_WIDTH: usize = 890;
const WINDOW_HEIGHT: usize = 350;

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

    let mut window = Window::new("KeyLayout", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default()).expect("Failed to create window");

    let mut background_frame = vec![0x00000000; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut buffer = vec![0x00000000; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut active_keys = HashSet::new();

    let font_data = include_bytes!("fonts/OpenSans-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to load font");
    let mut font_settings_size = 64;

    let key_map = vec![
        (Key::F1,            50,   0, 50, 50, "F1"),
        (Key::F2,            110,  0, 50, 50, "F2"),
        (Key::F3,            170, 0, 50, 50, "F3"),
        (Key::F4,            230, 0, 50, 50, "F4"),
        (Key::F5,            320, 0, 50, 50, "F5"),
        (Key::F6,            380, 0, 50, 50, "F6"),
        (Key::F7,            440, 0, 50, 50, "F7"),
        (Key::F8,            500, 0, 50, 50, "F8"),
        (Key::F9,            590, 0, 50, 50, "F9"),
        (Key::F10,           650, 0, 50, 50, "F10"),
        (Key::F11,           710, 0, 50, 50, "F11"),
        (Key::F12,           770, 0, 50, 50, "F12"),
    
        (Key::Num1,          0,   60,  50,  50,  "1"),
        (Key::Num2,          60,  60,  50,  50,  "2"),
        (Key::Num3,          120, 60,  50,  50,  "3"),
        (Key::Num4,          180, 60,  50,  50,  "4"),
        (Key::Num5,          240, 60,  50,  50,  "5"),
        (Key::Num6,          300, 60,  50,  50,  "6"),
        (Key::Num7,          360, 60,  50,  50,  "7"),
        (Key::Num8,          420, 60,  50,  50,  "8"),
        (Key::Num9,          480, 60,  50,  50,  "9"),
        (Key::Num0,          540, 60,  50,  50,  "0"),
        (Key::Minus,         600, 60,  50,  50,  "-"),
        (Key::Equal,         660, 60,  50,  50,  "="),
        (Key::Backspace,     720, 60, 135,  50,  "Backspace"),

        (Key::Tab,           0,  120,  80,  50,  "Tab"),
        (Key::KeyQ,          90,  120,  50,  50,  "q"),
        (Key::KeyW,          150, 120,  50,  50,  "w"),
        (Key::KeyE,          210, 120,  50,  50,  "e"),
        (Key::KeyR,          270, 120,  50,  50,  "r"),
        (Key::KeyT,          330, 120,  50,  50,  "t"),
        (Key::KeyY,          390, 120,  50,  50,  "y"),
        (Key::KeyU,          450, 120,  50,  50,  "u"),
        (Key::KeyI,          510, 120,  50,  50,  "i"),
        (Key::KeyO,          570, 120,  50,  50,  "o"),
        (Key::KeyP,          630, 120,  50,  50,  "p"),
        (Key::LeftBracket,   690, 120,  50,  50,  "["),
        (Key::RightBracket,  750, 120,  50,  50,  "]"),
        (Key::BackSlash,     810, 120,  80,  50,  "\\"),

        (Key::CapsLock,      0,  180, 90,  50,  "Caps"),
        (Key::KeyA,          100, 180, 50,  50,  "a"),
        (Key::KeyS,          160, 180, 50,  50,  "s"),
        (Key::KeyD,          220, 180, 50,  50,  "d"),
        (Key::KeyF,          280, 180, 50,  50,  "f"),
        (Key::KeyG,          340, 180, 50,  50,  "g"),
        (Key::KeyH,          400, 180, 50,  50,  "h"),
        (Key::KeyJ,          460, 180, 50,  50,  "j"),
        (Key::KeyK,          520, 180, 50,  50,  "k"),
        (Key::KeyL,          580, 180, 50,  50,  "l"),
        (Key::SemiColon,     640, 180, 50,  50,  ";"),
        (Key::Quote,         700, 180, 50,  50,  "'"),
        (Key::Return,        760, 180, 130, 50,  "Enter"),

        (Key::ShiftLeft,     0,  240, 110, 50,  "Shift"),
        (Key::KeyZ,          120, 240, 50,  50,  "z"),
        (Key::KeyX,          180, 240, 50,  50,  "x"),
        (Key::KeyC,          240, 240, 50,  50,  "c"),
        (Key::KeyV,          300, 240, 50,  50,  "v"),
        (Key::KeyB,          360, 240, 50,  50,  "b"),
        (Key::KeyN,          420, 240, 50,  50,  "n"),
        (Key::KeyM,          480, 240, 50,  50,  "m"),
        (Key::Comma,         540, 240, 50,  50,  ","),
        (Key::Dot,           600, 240, 50,  50,  "."),
        (Key::Slash,         660, 240, 50,  50,  "/"),
        (Key::ShiftRight,    720, 240, 170, 50,  "Shift"),

        (Key::ControlLeft,   0,  300, 80,  50,  "Ctrl"),
        (Key::Alt,           90,  300, 80,  50,  "Alt"),
        (Key::Space,         180, 300, 400, 50,  "Space"),
        (Key::AltGr,         590, 300, 80,  50,  "Alt"),
        (Key::ControlRight,  680, 300, 80,  50,  "Ctrl"),
    ];

    let mut rectangle_color = config.rectangle_color;
    let mut rgb_component = ' ';

    let mut scroll_toggle: u8 = 0;

    let mut flash_timer: Option<(Instant, &'static str)> = None;
    let flash_duration = Duration::from_secs(2);

    for &(_key, x, y, width, height, _) in &key_map {
        rectangle(&mut background_frame, x, y, width, height, rectangle_color);
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
                        font_settings_size = 64;
                        flash_timer = Some((Instant::now(), "Component selected: Red"))
                    },
                    minifbKey::G => {
                        rgb_component = 'G';
                        font_settings_size = 64;
                        flash_timer = Some((Instant::now(), "Component selected: Green"))
                    },
                    minifbKey::B => {
                        rgb_component = 'B';
                        font_settings_size = 64;
                        flash_timer = Some((Instant::now(), "Component selected: Blue"))
                    },
                    _ => {
                        font_settings_size = 40;
                        flash_timer = Some((Instant::now(), "Valid components: R - Red, G - Green, B - Blue"))
                        
                    }
                }
            );
            
            if let Some(mouse_wheel) = window.get_scroll_wheel() {
                let scroll_amount = mouse_wheel.1 as i32;

                scroll_color(&mut rectangle_color, scroll_amount, rgb_component, &mut flash_timer);

                background_frame.fill(0);

                for &(_key, x, y, width, height, _) in &key_map {
                    rectangle(&mut background_frame, x, y, width, height, rectangle_color);
                }
                    
                config.rectangle_color = rectangle_color;
                config.save_to_file("KeyLayout.ini");
            }
        }

        if now.duration_since(last_frame) >= frame_time {
            let shift_pressed = active_keys.contains(&Key::ShiftLeft) || active_keys.contains(&Key::ShiftRight);

            if window.get_mouse_down(MouseButton::Middle) && scroll_toggle == 0 {
                scroll_toggle = 1;
                font_settings_size = 64;
                flash_timer = Some((Instant::now(), "Scroll toggle: ON"));
                thread::sleep(Duration::from_millis(125));
            }
            else if window.get_mouse_down(MouseButton::Middle) && scroll_toggle == 1 {
                scroll_toggle = 0;
                font_settings_size = 64;
                flash_timer = Some((Instant::now(), "Scroll toggle: OFF"));
                thread::sleep(Duration::from_millis(125));
            }

            buffer.copy_from_slice(&background_frame);

            for &(key, x, y, width, height, label) in &key_map {
                if active_keys.contains(&key) {
                    rectangle(&mut buffer, x, y, width, height, 0xFFFFFFFF);
                }

                let text = if shift_pressed && label.len() == 1 {
                    label.to_uppercase()
                } else {
                    label.to_string()
                };

                key_text(&font, &mut buffer, x, y, width, height, &text, &mut glyph_cache);
            }

            if let Some((start, msg)) = flash_timer {
                if start.elapsed() <= flash_duration {
                    settings_text(&font, font_settings_size, &mut buffer, msg, &mut glyph_cache);
                } else {
                    flash_timer = None;
                }
            }

            window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).expect("Failed to update buffer");
            last_frame = Instant::now();
        } else {
            window.update();
        }
    }
}

fn scroll_color(color: &mut u32, scroll_amount: i32, component: char, flash_timer: &mut Option<(Instant, &'static str)>) {
    let mut red = ((*color >> 16) & 0xFF) as i32;
    let mut green = ((*color >> 8) & 0xFF) as i32;
    let mut blue = (*color & 0xFF) as i32;

    match component {
        'R' => red = (red + scroll_amount).clamp(0x0, 0xFF),
        'G' => green = (green + scroll_amount).clamp(0x0, 0xFF),
        'B' => blue = (blue + scroll_amount).clamp(0x0, 0xFF),
        _ => {
            *flash_timer = Some((Instant::now(), "Pick RGB component first !"));
            return;
        }
    }

    *color = (0xFF << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32);
}
