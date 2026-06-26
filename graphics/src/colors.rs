//! ============================================================
//! NEXORA Graphics Engine
//! colors.rs
//! ------------------------------------------------------------
//! Official NEXORA Color Palette
//! ============================================================

use crate::renderer::Color;

/// Main color palette.
pub struct Palette;

impl Palette {

    /// Background
    pub const BACKGROUND: Color = Color {
        r: 10,
        g: 10,
        b: 15,
        a: 255,
    };

    /// Pure White
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    /// Aurora Blue
    pub const AURORA_BLUE: Color = Color {
        r: 0,
        g: 220,
        b: 255,
        a: 255,
    };

    /// Aurora Purple
    pub const AURORA_PURPLE: Color = Color {
        r: 165,
        g: 70,
        b: 255,
        a: 255,
    };

    /// Accent Gold
    pub const GOLD: Color = Color {
        r: 255,
        g: 210,
        b: 60,
        a: 255,
    };

    /// Success
    pub const SUCCESS: Color = Color {
        r: 0,
        g: 200,
        b: 120,
        a: 255,
    };

    /// Warning
    pub const WARNING: Color = Color {
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    };

    /// Error
    pub const ERROR: Color = Color {
        r: 230,
        g: 50,
        b: 50,
        a: 255,
    };

}
