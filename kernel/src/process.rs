//! ============================================================
//! NEXORA OS Kernel
//! process.rs
//! ------------------------------------------------------------
//! Handles process creation and management.
//!
//! Future Responsibilities:
//! - Create processes
//! - Terminate processes
//! - Suspend and resume processes
//! - Process states
//! - Resource management
//! ============================================================

/// Represents the state of a process.
#[derive(Debug, Clone, Copy)]
pub enum ProcessState {
    Ready,
    Running,
    Waiting,
    Suspended,
    Terminated,
}

/// Represents a process.
pub struct Process {
    pub id: u32,
    pub name: String,
    pub state: ProcessState,
}

impl Process {

    /// Create a new process.
    pub fn new(id: u32, name: &str) -> Self {

        Self {

            id,

            name: name.to_string(),

            state: ProcessState::Ready,

        }

    }

    /// Start the process.
    pub fn start(&mut self) {

        self.state = ProcessState::Running;

        println!(
            "[PROCESS] '{}' started.",
            self.name
        );

    }

    /// Suspend the process.
    pub fn suspend(&mut self) {

        self.state = ProcessState::Suspended;

        println!(
            "[PROCESS] '{}' suspended.",
            self.name
        );

    }

    /// Stop the process.
    pub fn terminate(&mut self) {

        self.state = ProcessState::Terminated;

        println!(
            "[PROCESS] '{}' terminated.",
            self.name
        );

    }

}

/// Initializes the process manager.
pub fn initialize() {

    println!("[PROCESS] Initializing Process Manager...");

    let system = Process::new(0, "System");

    println!(
        "[PROCESS] '{}' created with PID {}.",
        system.name,
        system.id
    );

    println!("[PROCESS] Process Manager online.");

}
