//! ============================================================
//! NEXORA Graphics Engine
//! compositor.rs
//! ------------------------------------------------------------
//! Screen Compositor
//!
//! Responsibilities
//! - Layer composition
//! - Window stacking
//! - Transparency
//! - Blur effects
//! - Frame presentation
//! ============================================================

/// Represents a drawable layer.
#[derive(Clone)]
pub struct Layer {

    pub id: u32,

    pub name: String,

    pub visible: bool,

}

impl Layer {

    pub fn new(
        id: u32,
        name: &str,
    ) -> Self {

        Self {

            id,

            name: name.to_string(),

            visible: true,

        }

    }

}

/// Compositor.
pub struct Compositor {

    layers: Vec<Layer>,

}

impl Compositor {

    /// Create compositor.
    pub fn new() -> Self {

        Self {

            layers: Vec::new(),

        }

    }

    /// Initialize.
    pub fn initialize(&self) {

        println!("[COMPOSITOR] Initialized.");

    }

    /// Add layer.
    pub fn add_layer(
        &mut self,
        layer: Layer,
    ) {

        println!(
            "[COMPOSITOR] Added '{}'",
            layer.name
        );

        self.layers.push(layer);

    }

    /// Render all visible layers.
    pub fn render(&self) {

        println!();

        println!("========== COMPOSITOR ==========");

        for layer in &self.layers {

            if layer.visible {

                println!(
                    "Rendering Layer: {}",
                    layer.name
                );

            }

        }

        println!("Frame Complete");

        println!("===============================");

    }

}
