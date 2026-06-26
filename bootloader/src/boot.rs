//! NEXORA OS Bootloader
//! boot.rs
//!
//! This module controls the boot sequence of the NEXORA OS bootloader.

use crate::splash;

/// Represents the current stage of the boot process.
#[derive(Debug)]
pub enum BootStage {
    PowerOn,
    InitializeHardware,
    ShowSplash,
    VerifyKernel,
    LoadKernel,
    StartKernel,
}

/// Boot manager responsible for executing the startup sequence.
pub struct BootManager {
    current_stage: BootStage,
}

impl BootManager {
    /// Creates a new boot manager.
    pub fn new() -> Self {
        Self {
            current_stage: BootStage::PowerOn,
        }
    }

    /// Starts the boot sequence.
    pub fn start(&mut self) {
        self.initialize_hardware();
        self.show_splash();
        self.verify_kernel();
        self.load_kernel();
        self.start_kernel();
    }

    fn initialize_hardware(&mut self) {
        self.current_stage = BootStage::InitializeHardware;
        println!("[BOOT] Initializing hardware...");
    }

    fn show_splash(&mut self) {
        self.current_stage = BootStage::ShowSplash;
        println!("[BOOT] Displaying NEXORA splash screen...");
        splash::display();
    }

    fn verify_kernel(&mut self) {
        self.current_stage = BootStage::VerifyKernel;
        println!("[BOOT] Verifying kernel...");
    }

    fn load_kernel(&mut self) {
        self.current_stage = BootStage::LoadKernel;
        println!("[BOOT] Loading kernel...");
    }

    fn start_kernel(&mut self) {
        self.current_stage = BootStage::StartKernel;
        println!("[BOOT] Launching kernel...");
    }
}
