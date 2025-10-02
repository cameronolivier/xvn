//! Interactive setup wizard for xvn
//!
//! This module provides an interactive configuration wizard that guides
//! users through initial setup with auto-detection, validation, and
//! educational prompts.

pub mod wizard;
pub mod prompts;
pub mod detection;
pub mod validation;

pub use wizard::{run_interactive_wizard, run_quick_setup, run_non_interactive};
use anyhow::Result;

/// Main entry point for the init command
pub fn init(quick: bool, non_interactive: bool, force: bool) -> Result<()> {
    if non_interactive {
        run_non_interactive(force)
    } else if quick {
        run_quick_setup(force)
    } else {
        run_interactive_wizard(force)
    }
}
