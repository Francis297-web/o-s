//! ============================================================
//! NEXORA Graphics Engine
//! renderer.rs
//! ------------------------------------------------------------
//! Responsible for rendering everything on screen.
//!
//! Future Responsibilities
//! - Text Rendering
//! - Shape Rendering
//! - Image Rendering
//! - Frame Presentation
//! - Glow Effects
//! - Aurora Ring
//! - UI Rendering
//! ============================================================

/// RGBA color.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {

    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const AURORA_BLUE: Color = Color {
        r: 0,
        g: 220,
        b: 255,
        a: 255,
    };

    pub const AURORA_PURPLE: Color = Color {
        r: 170,
        g: 70,
        b: 255,
        a: 255,
    };
}

/// Main graphics renderer.
pub struct Renderer {

    width: u32,

    height: u32,

}

impl Renderer {

    /// Create renderer.
    pub fn new(width: u32, height: u32) -> Self {

        Self {

            width,

            height,

        }

    }

    /// Initialize graphics.
    pub fn initialize(&self) {

        println!(
            "[GRAPHICS] Renderer initialized ({}x{})",
            self.width,
            self.height
        );

    }

    /// Clear the screen.
    pub fn clear(&self, color: Color) {

        println!(
            "[RENDERER] Clear Screen -> {:?}",
            color
        );

    }

    /// Draw text.
    pub fn draw_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) {

        println!(
            "[RENDERER] TEXT '{}' ({},{}) {:?}",
            text,
            x,
            y,
            color
        );

    }

    /// Draw line.
    pub fn draw_line(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: Color,
    ) {

        println!(
            "[RENDERER] LINE ({},{}) -> ({},{}) {:?}",
            x1,
            y1,
            x2,
            y2,
            color
        );

    }

    /// Draw rectangle.
    pub fn draw_rect(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Color,
    ) {

        println!(
            "[RENDERER] RECT ({},{}) {}x{} {:?}",
            x,
            y,
            width,
            height,
            color
        );

    }

    /// Draw circle.
    pub fn draw_circle(
        &self,
        x: i32,
        y: i32,
        radius: u32,
        color: Color,
    ) {

        println!(
            "[RENDERER] CIRCLE ({},{}) radius={} {:?}",
            x,
            y,
            radius,
            color
        );

    }

    /// Draw image.
    pub fn draw_image(
        &self,
        path: &str,
        x: i32,
        y: i32,
    ) {

        println!(
            "[RENDERER] IMAGE '{}' ({},{})",
            path,
            x,
            y
        );

    }

    /// Draw glow effect.
    pub fn draw_glow(
        &self,
        intensity: f32,
    ) {

        println!(
            "[RENDERER] Glow intensity {}",
            intensity
        );

    }

    /// Draw Aurora Ring.
    pub fn draw_aurora_ring(
        &self,
        radius: u32,
    ) {

        println!(
            "[RENDERER] Aurora Ring radius {}",
            radius
        );

    }

    /// Present the completed frame.
    pub fn present(&self) {

        println!(
            "[GRAPHICS] Present Frame"
        );

    }

}
