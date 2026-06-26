//! ============================================================
//! NEXORA OS Kernel
//! scheduler.rs
//! ------------------------------------------------------------
//! CPU Process Scheduler
//!
//! Responsibilities
//! - Schedule processes
//! - Allocate CPU time
//! - Switch between tasks
//! - Manage process priorities
//! ============================================================

use crate::process::{Process, ProcessState};

/// Represents the priority of a process.
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

/// Scheduler responsible for managing CPU execution.
pub struct Scheduler {
    processes: Vec<Process>,
}

impl Scheduler {

    /// Creates a new scheduler.
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    /// Starts the scheduler.
    pub fn initialize(&self) {
        println!("[SCHEDULER] CPU Scheduler initialized.");
    }

    /// Adds a process to the scheduling queue.
    pub fn add_process(&mut self, process: Process) {
        println!(
            "[SCHEDULER] Added '{}' to scheduling queue.",
            process.name
        );

        self.processes.push(process);
    }

    /// Executes all ready processes.
    pub fn run(&mut self) {

        println!("[SCHEDULER] Starting task scheduling...");

        for process in &mut self.processes {

            if matches!(process.state, ProcessState::Ready) {

                process.start();

            }

        }

        println!("[SCHEDULER] Scheduling complete.");

    }

    /// Displays scheduler status.
    pub fn status(&self) {

        println!();
        println!("========== Scheduler ==========");

        println!(
            "Active Processes: {}",
            self.processes.len()
        );

        println!("===============================");

    }

}

/// Kernel initialization entry point.
pub fn initialize() {

    println!("[SCHEDULER] Initializing Scheduler...");

    let mut scheduler = Scheduler::new();

    scheduler.initialize();

    scheduler.add_process(
        Process::new(1, "Aurora UI")
    );

    scheduler.add_process(
        Process::new(2, "System Services")
    );

    scheduler.add_process(
        Process::new(3, "Nexora AI")
    );

    scheduler.run();

    scheduler.status();

}
