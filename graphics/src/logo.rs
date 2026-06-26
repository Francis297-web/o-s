//! ============================================================
//! NEXORA Graphics Engine
//! logo.rs
//! ------------------------------------------------------------
//! Official NEXORA Logo Renderer
//!
//! Responsibilities
//! - Draw NEXORA text
//! - Draw Aurora Ring
//! - Draw Butterfly Logo
//! - Support animations
//! - Support scaling
//! ============================================================

use crate::renderer::{Color, Renderer};

/// Official NEXORA logo.
pub struct Logo {

    pub x: i32,

    pub y: i32,

    pub scale: f32,

}

impl Logo {

    /// Create logo.
    pub fn new(
        x: i32,
        y: i32,
        scale: f32,
    ) -> Self {

        Self {

            x,

            y,

            scale,

        }

    }

    /// Draw complete logo.
    pub fn draw(
        &self,
        renderer: &Renderer,
    ) {

        self.draw_text(renderer);

        self.draw_ring(renderer);

        self.draw_butterfly(renderer);

    }

    /// Draw NEXORA text.
    fn draw_text(
        &self,
        renderer: &Renderer,
    ) {

        renderer.draw_text(
            "NEXORA",
            self.x,
            self.y,
            Color::AURORA_BLUE,
        );

    }

    /// Draw Aurora Ring.
    fn draw_ring(
        &self,
        renderer: &Renderer,
    ) {

        let radius =
            (180.0 * self.scale) as u32;

        renderer.draw_aurora_ring(radius);

    }

    /// Draw butterfly.
    fn draw_butterfly(
        &self,
        renderer: &Renderer,
    ) {

        renderer.draw_text(

            "🦋",

            self.x,

            self.y - 140,

            Color::AURORA_PURPLE,

        );

    }

    /// Animate text appearing.
    pub fn animate_letters(
        &self,
        renderer: &Renderer,
    ) {

        let letters = [
            "N",
            "NE",
            "NEX",
            "NEXO",
            "NEXOR",
            "NEXORA",
        ];

        for text in letters {

            renderer.clear(Color::BLACK);

            renderer.draw_text(

                text,

                self.x,

                self.y,

                Color::AURORA_BLUE,

            );

            renderer.present();

        }

    }

    /// Glow animation.
    pub fn glow(
        &self,
        renderer: &Renderer,
    ) {

        renderer.draw_glow(0.25);

        renderer.draw_glow(0.50);

        renderer.draw_glow(0.75);

        renderer.draw_glow(1.00);

        renderer.present();

    }

    /// Aurora Ring expansion.
    pub fn expand_ring(
        &self,
        renderer: &Renderer,
    ) {

        for radius in (80..220).step_by(10) {

            renderer.draw_aurora_ring(radius);

            renderer.present();

        }

    }

}
