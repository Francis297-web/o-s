//! NEXORA OS Bootloader
//! main.rs
//!
//! Entry point for the NEXORA OS bootloader.

mod boot;
mod splash;

use boot::BootManager;

fn main() {
    println!("========================================");
    println!("        NEXORA OS BOOTLOADER");
    println!("========================================");
    println!("Version: v0.0.1");
    println!();

    let mut bootloader = BootManager::new();

    bootloader.start();

    println!();
    println!("Boot sequence completed.");
}
