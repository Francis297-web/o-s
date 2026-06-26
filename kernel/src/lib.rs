//! ============================================================
//! NEXORA OS Kernel
//! lib.rs
//! ------------------------------------------------------------
//! Main entry point for the NEXORA OS Kernel.
//!
//! Responsibilities:
//! - Initialize core kernel subsystems
//! - Start memory management
//! - Initialize interrupt handling
//! - Start process scheduler
//! - Prepare system services
//! ============================================================

pub mod interrupt;
pub mod memory;
pub mod power;
pub mod process;
pub mod scheduler;
pub mod syscall;

/// Represents the NEXORA OS Kernel.
pub struct Kernel;

impl Kernel {

    /// Creates a new kernel instance.
    pub fn new() -> Self {
        Kernel
    }

    /// Starts the kernel.
    pub fn initialize(&self) {

        println!("======================================");
        println!("        NEXORA OS KERNEL");
        println!("======================================");
        println!("Version: v0.0.1");
        println!();

        println!("[KERNEL] Boot sequence started...");

        memory::initialize();

        let interrupt_manager = interrupt::InterruptManager::new();
        interrupt_manager.initialize();

        process::initialize();

        scheduler::initialize();

        syscall::initialize();

        power::initialize();

        println!();
        println!("[KERNEL] All subsystems initialized.");
        println!("[KERNEL] Kernel is now running.");

    }

}
