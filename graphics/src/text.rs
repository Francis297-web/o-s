#![no_std]

use crate::display::{Display, Color};

/// Simple 8x8 bitmap font (ASCII 32–127)
/// Each character = 8 bytes (one byte per row)
pub const FONT: [[u8; 8]; 96] = include!("font8x8_basic.in");

pub struct Text;

impl Text {
    /// Draw a single character
    pub fn draw_char(
        display: &Display,
        x: usize,
        y: usize,
        ch: char,
        color: Color,
    ) {
        let index = match (ch as usize).checked_sub(32) {
            Some(i) if i < 96 => i,
            _ => return,
        };

        let glyph = FONT[index];

        for row in 0..8 {
            let line = glyph[row];
            for col in 0..8 {
                if (line >> col) & 1 == 1 {
                    display.put_pixel(x + col, y + row, color);
                }
            }
        }
    }

    /// Draw a string
    pub fn draw_string(
        display: &Display,
        mut x: usize,
        mut y: usize,
        text: &str,
        color: Color,
    ) {
        for ch in text.chars() {
            if ch == '\n' {
                y += 10;
                x = 0;
                continue;
            }

            Self::draw_char(display, x, y, ch, color);
            x += 8;
        }
    }

    /// Center text horizontally
    pub fn draw_centered(
        display: &Display,
        y: usize,
        text: &str,
        color: Color,
    ) {
        let width = display.width();
        let text_width = text.len() * 8;

        let x = if width > text_width {
            (width - text_width) / 2
        } else {
            0
        };

        Self::draw_string(display, x, y, text, color);
    }

    /// Simple terminal-like print (no scrolling yet)
    pub fn draw_terminal(
        display: &Display,
        lines: &[&str],
        start_x: usize,
        start_y: usize,
        color: Color,
    ) {
        let mut y = start_y;

        for line in lines {
            Self::draw_string(display, start_x, y, line, color);
            y += 10;
        }
    }
}
