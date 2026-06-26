//! ============================================================
//! NEXORA OS Kernel
//! interrupt.rs
//! ------------------------------------------------------------
//! Handles CPU interrupts and exceptions.
//!
//! Future responsibilities:
//! - Interrupt Descriptor Table (IDT)
//! - Exception handling
//! - Timer interrupts
//! - Keyboard/Touch interrupts
//! - Hardware IRQ handling
//! - System timer
//! ============================================================

/// Represents an interrupt that can occur in the system.
#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    DivideByZero,
    InvalidOpcode,
    PageFault,
    GeneralProtectionFault,
    Timer,
    Keyboard,
    Touchscreen,
    Display,
    Battery,
    Network,
    Unknown,
}

/// Interrupt Manager
pub struct InterruptManager;

impl InterruptManager {

    /// Creates a new interrupt manager.
    pub fn new() -> Self {
        InterruptManager
    }

    /// Initializes interrupt handling.
    pub fn initialize(&self) {
        println!("[KERNEL] Initializing Interrupt Manager...");
        println!("[KERNEL] Building Interrupt Descriptor Table...");
        println!("[KERNEL] Registering exception handlers...");
        println!("[KERNEL] Interrupt system online.");
    }

    /// Handles an incoming interrupt.
    pub fn handle(&self, interrupt: Interrupt) {

        match interrupt {

            Interrupt::DivideByZero =>
                println!("[INTERRUPT] Divide By Zero Exception"),

            Interrupt::InvalidOpcode =>
                println!("[INTERRUPT] Invalid Opcode"),

            Interrupt::PageFault =>
                println!("[INTERRUPT] Page Fault"),

            Interrupt::GeneralProtectionFault =>
                println!("[INTERRUPT] General Protection Fault"),

            Interrupt::Timer =>
                println!("[INTERRUPT] System Timer Tick"),

            Interrupt::Keyboard =>
                println!("[INTERRUPT] Keyboard Event"),

            Interrupt::Touchscreen =>
                println!("[INTERRUPT] Touchscreen Event"),

            Interrupt::Display =>
                println!("[INTERRUPT] Display Refresh"),

            Interrupt::Battery =>
                println!("[INTERRUPT] Battery Status Changed"),

            Interrupt::Network =>
                println!("[INTERRUPT] Network Packet Received"),

            Interrupt::Unknown =>
                println!("[INTERRUPT] Unknown Interrupt"),
        }

    }

    /// Shuts down interrupt handling.
    pub fn shutdown(&self) {
        println!("[KERNEL] Interrupt Manager shutdown.");
    }

}
