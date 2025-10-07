//! Interactive setup wizard for xvn
//!
//! This module provides an interactive configuration wizard that guides
//! users through initial setup with auto-detection, validation, and
//! educational prompts.

pub mod detection;
pub mod prompts;
pub mod validation;
pub mod wizard;

use anyhow::Result;
pub use wizard::{run_interactive_wizard, run_non_interactive, run_quick_setup};

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
