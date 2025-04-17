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
    let total_text_width: f32 = text.chars().map(|c| {
        let (metrics, _) = *glyph_cache.entry((c, 24)).or_insert_with(|| font.rasterize(c, 24.0));
        metrics.advance_width
    }).sum();

    let start_x = x as f32 + (width as f32 - total_text_width) / 2.0;
    let key_center = y as f32 + height as f32 / 2.0;

    let mut x_cursor = start_x;
    for c in text.chars() {
        let (metrics, bitmap) = glyph_cache.entry((c, 24)).or_insert_with(|| font.rasterize(c, 24.0));

        let glyph_center = metrics.height as f32 / 2.0;
        let char_y = key_center - glyph_center - metrics.ymin as f32;
        let char_x = x_cursor + metrics.xmin as f32;

        let char_x = char_x.round() as usize;
        let char_y = char_y.round() as usize;

        for (dy, row) in bitmap.chunks(metrics.width).enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                let px = char_x + dx;
                let py = char_y + dy;
                if px < WINDOW_WIDTH && py < WINDOW_HEIGHT {
                    unsafe {
                        let idx = py * WINDOW_WIDTH + px;
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

pub fn settings_text(
    font: &Font,
    font_settings_size: u32,
    buffer: &mut [u32],
    text: &str,
    glyph_cache: &mut HashMap<(char, u32), (Metrics, Vec<u8>)>,
) {
    let total_width: f32 = text.chars().map(|c| {
        let key = (c, font_settings_size);
        let (metrics, _) = glyph_cache
            .entry(key)
            .or_insert_with(|| font.rasterize(c, font_settings_size as f32));
        metrics.advance_width
    }).sum();

    let mut x_cursor = ((WINDOW_WIDTH as f32) - total_width) / 2.0;
    let center_y = (WINDOW_HEIGHT as f32) / 2.0;

    for c in text.chars() {
        let key = (c, font_settings_size);
        let (metrics, bitmap) = glyph_cache
            .entry(key)
            .or_insert_with(|| font.rasterize(c, font_settings_size as f32));

        if metrics.width == 0 || metrics.height == 0 {
            x_cursor += metrics.advance_width;
            continue;
        }

        let glyph_x = (x_cursor + metrics.xmin as f32).round() as usize;
        let glyph_y = (center_y - (metrics.height as f32 / 2.0) - metrics.ymin as f32).round() as usize;

        for (row_idx, row) in bitmap.chunks(metrics.width).enumerate() {
            for (col_idx, &pixel) in row.iter().enumerate() {
                let px = glyph_x + col_idx;
                let py = glyph_y + row_idx;
                if px < WINDOW_WIDTH && py < WINDOW_HEIGHT {
                    unsafe {
                        let idx = py * WINDOW_WIDTH + px;
                        let alpha = pixel as f32 / 255.0;

                        let dest_color = *buffer.get_unchecked_mut(idx);
                        let dest_r = ((dest_color >> 16) & 0xFF) as f32;
                        let dest_g = ((dest_color >> 8) & 0xFF) as f32;
                        let dest_b = (dest_color & 0xFF) as f32;

                        let blended_r = (0.0 * alpha + dest_r * (1.0 - alpha)) as u32;
                        let blended_g = (255.0 * alpha + dest_g * (1.0 - alpha)) as u32;
                        let blended_b = (0.0 * alpha + dest_b * (1.0 - alpha)) as u32;

                        *buffer.get_unchecked_mut(idx) = (0xFF << 24) | (blended_r << 16) | (blended_g << 8) | blended_b;
                    }
                }
            }
        }

        x_cursor += metrics.advance_width;
    }
}
