#![no_std]

use crate::display::{Display, Color};

pub struct Effects;

impl Effects {
    /// Fade screen to black (or any color)
    pub fn fade_out(display: &Display, color: Color, steps: usize) {
        let (w, h) = (display.width(), display.height());

        for i in 0..steps {
            let alpha = i as u32 * 255 / steps as u32;
            let fade_color = blend(color, 0xFF000000, alpha);

            for y in 0..h {
                for x in 0..w {
                    display.put_pixel(x, y, fade_color);
                }
            }
        }
    }

    /// Fade in from black (or any color)
    pub fn fade_in(display: &Display, color: Color, steps: usize) {
        let (w, h) = (display.width(), display.height());

        for i in (0..steps).rev() {
            let alpha = i as u32 * 255 / steps as u32;
            let fade_color = blend(color, 0xFF000000, alpha);

            for y in 0..h {
                for x in 0..w {
                    display.put_pixel(x, y, fade_color);
                }
            }
        }
    }

    /// Flash the screen (like system alert / crash / boot effect)
    pub fn flash(display: &Display, color: Color, intensity: usize) {
        let (w, h) = (display.width(), display.height());

        for _ in 0..intensity {
            for y in 0..h {
                for x in 0..w {
                    display.put_pixel(x, y, color);
                }
            }
        }
    }

    /// Simple box blur (VERY expensive but good for OS prototype effects)
    pub fn blur(display: &Display, radius: usize) {
        let w = display.width();
        let h = display.height();

        let mut buffer: alloc::vec::Vec<Color> = alloc::vec::Vec::with_capacity(w * h);

        // copy framebuffer into buffer
        for y in 0..h {
            for x in 0..w {
                unsafe {
                    let pixel = *display.framebuffer.add(y * display.stride + x);
                    buffer.push(pixel);
                }
            }
        }

        for y in 0..h {
            for x in 0..w {
                let mut r = 0u32;
                let mut g = 0u32;
                let mut b = 0u32;
                let mut count = 0u32;

                for dy in -(radius as isize)..=(radius as isize) {
                    for dx in -(radius as isize)..=(radius as isize) {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && ny >= 0 && nx < w as isize && ny < h as isize {
                            let idx = ny as usize * w + nx as usize;
                            let c = buffer[idx];

                            r += (c >> 16) & 0xFF;
                            g += (c >> 8) & 0xFF;
                            b += c & 0xFF;
                            count += 1;
                        }
                    }
                }

                let r = (r / count) & 0xFF;
                let g = (g / count) & 0xFF;
                let b = (b / count) & 0xFF;

                let final_color = (0xFF << 24) | (r << 16) | (g << 8) | b;
                display.put_pixel(x, y, final_color);
            }
        }
    }

    /// Invert screen colors (retro effect / debugging tool)
    pub fn invert(display: &Display) {
        let w = display.width();
        let h = display.height();

        for y in 0..h {
            for x in 0..w {
                unsafe {
                    let p = display.framebuffer.add(y * display.stride + x);
                    let c = *p;

                    let a = c & 0xFF000000;
                    let r = 0xFF - ((c >> 16) & 0xFF);
                    let g = 0xFF - ((c >> 8) & 0xFF);
                    let b = 0xFF - (c & 0xFF);

                    *p = a | (r << 16) | (g << 8) | b;
                }
            }
        }
    }

    /// Dim screen (for overlays, menus, compositor prep)
    pub fn dim(display: &Display, factor: u8) {
        let w = display.width();
        let h = display.height();

        for y in 0..h {
            for x in 0..w {
                unsafe {
                    let p = display.framebuffer.add(y * display.stride + x);
                    let c = *p;

                    let a = c & 0xFF000000;
                    let r = ((c >> 16) & 0xFF) * factor as u32 / 255;
                    let g = ((c >> 8) & 0xFF) * factor as u32 / 255;
                    let b = (c & 0xFF) * factor as u32 / 255;

                    *p = a | (r << 16) | (g << 8) | b;
                }
            }
        }
    }
}

/// Blend helper (simple alpha blend)
fn blend(src: Color, dst: Color, alpha: u32) -> Color {
    let inv = 255 - alpha;

    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;

    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;

    let r = (sr * alpha + dr * inv) / 255;
    let g = (sg * alpha + dg * inv) / 255;
    let b = (sb * alpha + db * inv) / 255;

    0xFF000000 | (r << 16) | (g << 8) | b
}
