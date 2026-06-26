#![no_std]

use crate::display::{Display, Color};
use crate::text::Text;
use crate::effects::Effects;

/// Main graphics renderer (real framebuffer-based)
pub struct Renderer {
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn initialize(&self) {
        // In real OS: init framebuffer, GPU, or boot services
    }

    pub fn clear(&self, display: &Display, color: Color) {
        display.clear(color);
    }

    pub fn draw_text(
        &self,
        display: &Display,
        text: &str,
        x: usize,
        y: usize,
        color: Color,
    ) {
        Text::draw_string(display, x, y, text, color);
    }

    pub fn draw_centered_text(
        &self,
        display: &Display,
        text: &str,
        y: usize,
        color: Color,
    ) {
        Text::draw_centered(display, y, text, color);
    }

    pub fn draw_rect(
        &self,
        display: &Display,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: Color,
    ) {
        display.draw_rect(x, y, w, h, color);
    }

    pub fn draw_pixel(
        &self,
        display: &Display,
        x: usize,
        y: usize,
        color: Color,
    ) {
        display.put_pixel(x, y, color);
    }

    pub fn apply_glow(&self, display: &Display) {
        // placeholder hook for future shader-like effect
        Effects::dim(display, 200);
    }

    pub fn present(&self) {
        // In real OS: sync framebuffer / GPU / vsync
    }
}
