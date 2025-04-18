use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};
use {std::collections::HashMap, fontdue::{Font, Metrics}};

pub fn rectangle(buffer: &mut [u32], x: usize, y: usize, width: usize, height: usize, color: u32) {
    for row in 0..height {
        for col in 0..width {
            let idx = (y + row) * WINDOW_WIDTH + (x + col);
            unsafe { *buffer.get_unchecked_mut(idx) = color; }
        }
    }
}

pub fn key_text(
    font: &Font,
    buffer: &mut [u32],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    text: &str,
    glyph_cache: &mut HashMap<(char, u32), (Metrics, Vec<u8>)>,
) {
    const SIZE: u32 = 24;
    const COLOR: (f32, f32, f32) = (200.0, 200.0, 200.0);

    let mut glyphs = Vec::with_capacity(text.len());
    let mut total_width = 0.0;

    for c in text.chars() {
        let entry = glyph_cache.entry((c, SIZE)).or_insert_with(|| font.rasterize(c, SIZE as f32)).clone();
        total_width += entry.0.advance_width;
        glyphs.push((c, entry));
    }

    let start_x = x as f32 + (width as f32 - total_width) * 0.5;
    let center_y = y as f32 + height as f32 * 0.5;

    let mut x_cursor = start_x;

    for (_c, (metrics, bitmap)) in glyphs {
        let x_pos = (x_cursor + metrics.xmin as f32).round() as usize;
        let y_pos = (center_y - (metrics.height as f32 * 0.5) - metrics.ymin as f32).round() as usize;

        let width = metrics.width;
        let height = metrics.height;

        for row in 0..height {
            let row_start = row * width;
            let py = y_pos + row;

            if py >= WINDOW_HEIGHT {
                continue;
            }

            for col in 0..width {
                let px = x_pos + col;

                if px >= WINDOW_WIDTH {
                    continue;
                }

                let pixel = bitmap[row_start + col];
                if pixel == 0 {
                    continue;
                }

                let alpha = pixel as f32 / 255.0;
                let idx = py * WINDOW_WIDTH + px;

                unsafe {
                    let dest_color = buffer.get_unchecked_mut(idx);
                    let dc = *dest_color;
                    let dest_r = ((dc >> 16) & 0xFF) as f32;
                    let dest_g = ((dc >> 8) & 0xFF) as f32;
                    let dest_b = (dc & 0xFF) as f32;

                    let r = (COLOR.0 * alpha + dest_r * (1.0 - alpha)) as u32;
                    let g = (COLOR.1 * alpha + dest_g * (1.0 - alpha)) as u32;
                    let b = (COLOR.2 * alpha + dest_b * (1.0 - alpha)) as u32;

                    *dest_color = (0xFF << 24) | (r << 16) | (g << 8) | b;
                }
            }
        }

        x_cursor += metrics.advance_width;
    }
}

pub fn settings_text(
    font: &Font,
    font_settings_size: u32,
    buffer: &mut [u32],
    text: &str,
    glyph_cache: &mut HashMap<(char, u32), (Metrics, Vec<u8>)>,
) {
    const COLOR: (f32, f32, f32) = (0.0, 255.0, 0.0);

    let mut glyphs = Vec::with_capacity(text.len());
    let mut total_width = 0.0;

    for c in text.chars() {
        let key = (c, font_settings_size);
        let (metrics, bitmap) = glyph_cache.entry(key).or_insert_with(|| font.rasterize(c, font_settings_size as f32)).clone();

        total_width += metrics.advance_width;
        glyphs.push((metrics, bitmap));
    }

    let mut x_cursor = (WINDOW_WIDTH as f32 - total_width) * 0.5;
    let center_y = WINDOW_HEIGHT as f32 * 0.5;

    for (metrics, bitmap) in glyphs {
        if metrics.width == 0 || metrics.height == 0 {
            x_cursor += metrics.advance_width;
            continue;
        }

        let glyph_x = (x_cursor + metrics.xmin as f32).round() as usize;
        let glyph_y = (center_y - (metrics.height as f32 * 0.5) - metrics.ymin as f32).round() as usize;

        for row in 0..metrics.height {
            let py = glyph_y + row;
            if py >= WINDOW_HEIGHT {
                continue;
            }

            let row_start = row * metrics.width;

            for col in 0..metrics.width {
                let px = glyph_x + col;
                if px >= WINDOW_WIDTH {
                    continue;
                }

                let pixel = bitmap[row_start + col];
                if pixel == 0 {
                    continue;
                }

                let alpha = pixel as f32 / 255.0;
                let idx = py * WINDOW_WIDTH + px;

                unsafe {
                    let dest_color = buffer.get_unchecked_mut(idx);
                    let dc = *dest_color;
                    let dest_r = ((dc >> 16) & 0xFF) as f32;
                    let dest_g = ((dc >> 8) & 0xFF) as f32;
                    let dest_b = (dc & 0xFF) as f32;

                    let r = (COLOR.0 * alpha + dest_r * (1.0 - alpha)) as u32;
                    let g = (COLOR.1 * alpha + dest_g * (1.0 - alpha)) as u32;
                    let b = (COLOR.2 * alpha + dest_b * (1.0 - alpha)) as u32;

                    *dest_color = (0xFF << 24) | (r << 16) | (g << 8) | b;
                }
            }
        }

        x_cursor += metrics.advance_width;
    }
}
