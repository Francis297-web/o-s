//! ============================================================
//! NEXORA OS Kernel
//! memory.rs
//! ------------------------------------------------------------
//! Handles system memory management.
//!
//! Future responsibilities:
//! - Physical Memory Manager (PMM)
//! - Virtual Memory Manager (VMM)
//! - Heap Allocation
//! - Memory Protection
//! ============================================================

/// Memory Manager
pub struct MemoryManager {
    total_memory: usize,
    used_memory: usize,
}

impl MemoryManager {

    /// Create a new memory manager.
    pub fn new() -> Self {
        Self {
            total_memory: 0,
            used_memory: 0,
        }
    }

    /// Initialize memory management.
    pub fn initialize(&mut self) {

        println!("[MEMORY] Detecting system memory...");

        self.total_memory = 4096;

        println!(
            "[MEMORY] {} MB detected.",
            self.total_memory
        );

        println!("[MEMORY] Memory Manager online.");

    }

    /// Allocate memory.
    pub fn allocate(&mut self, amount: usize) {

        self.used_memory += amount;

        println!(
            "[MEMORY] Allocated {} MB",
            amount
        );

    }

    /// Free memory.
    pub fn free(&mut self, amount: usize) {

        if self.used_memory >= amount {

            self.used_memory -= amount;

        }

        println!(
            "[MEMORY] Freed {} MB",
            amount
        );

    }

    /// Display current memory usage.
    pub fn status(&self) {

        println!(
            "[MEMORY] Used: {} MB / {} MB",
            self.used_memory,
            self.total_memory
        );

    }

}

/// Kernel entry point.
pub fn initialize() {

    let mut manager = MemoryManager::new();

    manager.initialize();

}
