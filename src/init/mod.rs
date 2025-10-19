//! Interactive setup wizard for anvs
//!
//! This module provides an interactive configuration wizard that guides
//! users through initial setup with auto-detection, validation, and
//! educational prompts.

pub mod detection;
pub mod prompts;
pub mod summary;
pub mod timeline;
pub mod validation;
pub mod wizard;

use anyhow::Result;
pub use wizard::{handle_init, run_interactive_wizard, run_non_interactive, run_quick_setup};

/// Main entry point for the init command
pub fn init(quick: bool, advanced: bool, non_interactive: bool, force: bool) -> Result<()> {
    if non_interactive {
        run_non_interactive(force)
    } else {
        // Route to appropriate wizard mode
        // Default behavior (no flags): quick mode
        handle_init(quick, advanced, force)
    }
}
