//! ============================================================
//! NEXORA Graphics Engine
//! framebuffer.rs
//! ------------------------------------------------------------
//! Software Framebuffer
//!
//! Responsibilities
//! - Pixel Storage
//! - Drawing Surface
//! - Double Buffering (future)
//! - GPU Upload (future)
//! ============================================================

/// Represents one RGBA pixel.
#[derive(Clone, Copy)]
pub struct Pixel {

    pub r: u8,

    pub g: u8,

    pub b: u8,

    pub a: u8,

}

impl Pixel {

    pub fn new(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Self {

        Self { r, g, b, a }

    }

}

/// Software framebuffer.
pub struct FrameBuffer {

    width: usize,

    height: usize,

    pixels: Vec<Pixel>,

}

impl FrameBuffer {

    /// Create framebuffer.
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {

        let pixels = vec![
            Pixel::new(0,0,0,255);
            width * height
        ];

        Self {

            width,

            height,

            pixels,

        }

    }

    /// Width
    pub fn width(&self) -> usize {

        self.width

    }

    /// Height
    pub fn height(&self) -> usize {

        self.height

    }

    /// Clear framebuffer.
    pub fn clear(
        &mut self,
        color: Pixel,
    ) {

        for pixel in self.pixels.iter_mut() {

            *pixel = color;

        }

    }

    /// Set one pixel.
    pub fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Pixel,
    ) {

        if x >= self.width || y >= self.height {

            return;

        }

        let index = y * self.width + x;

        self.pixels[index] = color;

    }

    /// Read one pixel.
    pub fn get_pixel(
        &self,
        x: usize,
        y: usize,
    ) -> Option<Pixel> {

        if x >= self.width || y >= self.height {

            return None;

        }

        Some(
            self.pixels[
                y * self.width + x
            ]
        )

    }

    /// Fill rectangle.
    pub fn fill_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Pixel,
    ) {

        for py in y..(y + height) {

            for px in x..(x + width) {

                self.set_pixel(px, py, color);

            }

        }

    }

}
