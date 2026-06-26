#![no_std]

use core::ptr;

/// A simple pixel format (ARGB8888)
pub type Color = u32;

/// Basic display structure backed by a raw framebuffer.
pub struct Display {
    width: usize,
    height: usize,
    stride: usize,
    framebuffer: *mut u32,
}

impl Display {
    /// Create a new display instance
    ///
    /// # Safety
    /// - framebuffer must be a valid writable pointer
    /// - size must match width * height (or stride * height)
    pub unsafe fn new(
        framebuffer: *mut u32,
        width: usize,
        height: usize,
        stride: usize,
    ) -> Self {
        Self {
            framebuffer,
            width,
            height,
            stride,
        }
    }

    /// Draw a single pixel
    pub fn put_pixel(&self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        unsafe {
            let offset = y * self.stride + x;
            ptr::write_volatile(self.framebuffer.add(offset), color);
        }
    }

    /// Fill entire screen with a color
    pub fn clear(&self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, color);
            }
        }
    }

    /// Draw a rectangle (useful for UI blocks / compositor testing)
    pub fn draw_rect(
        &self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: Color,
    ) {
        for j in 0..h {
            for i in 0..w {
                self.put_pixel(x + i, y + j, color);
            }
        }
    }

    /// Optional: flush (useful if later using buffered rendering)
    pub fn flush(&self) {
        // For now: nothing needed.
        // Later: sync framebuffer, GPU command queue, or cache flush.
    }

    /// Get width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get height
    pub fn height(&self) -> usize {
        self.height
    }
}
