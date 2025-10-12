//! Uninstall command - removes all xvn installations and configuration

use crate::installation_detector::InstallationDetector;
use crate::output;
use crate::setup::profile_modification;
use anyhow::{Context, Result};
use inquire::Confirm;
use owo_colors::OwoColorize;
use std::fs;

/// Uninstall xvn completely
pub fn uninstall(force: bool) -> Result<()> {
    println!();
    output::warning("⚠️  xvn Uninstall");
    println!();

    // Detect all installations
    let installations = InstallationDetector::detect_all();

    if installations.is_empty() {
        output::info("No external xvn installations detected.");
        println!();
    } else {
        output::info("Detected xvn installations:");
        for (method, path) in &installations {
            output::info(&format!("  • {} at {}", method.description(), path.display()));
        }
        println!();
    }

    // Show what will be removed
    output::info("This will remove:");
    output::info("  • ~/.xvn directory (all versions and binaries)");
    output::info("  • ~/.xvnrc configuration file");
    output::info("  • Shell integration from .bashrc and .zshrc");

    if !installations.is_empty() {
        println!();
        output::warning("Note: External packages must be uninstalled separately:");
        for (method, _) in &installations {
            output::info(&format!("  • {}", method.uninstall_command()));
        }
    }

    println!();

    // Confirm unless --force
    if !force {
        let confirmed = Confirm::new("Are you sure you want to uninstall xvn?")
            .with_default(false)
            .with_help_message("This action cannot be undone")
            .prompt()
            .context("Failed to get confirmation")?;

        if !confirmed {
            output::info("Uninstall cancelled.");
            return Ok(());
        }
        println!();
    }

    // Perform uninstallation
    let mut removed_items = Vec::new();

    // 1. Remove shell integration
    if let Ok(home) = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory")) {
        let shells = vec![
            (home.join(".bashrc"), "bash"),
            (home.join(".zshrc"), "zsh"),
        ];

        for (profile_path, shell_name) in shells {
            if profile_path.exists() {
                match profile_modification::remove_from_profile(&profile_path) {
                    Ok(true) => {
                        removed_items.push(format!("Shell integration from {}", shell_name));
                    }
                    Ok(false) => {
                        // Not present, skip
                    }
                    Err(e) => {
                        output::warning(&format!("Warning: Could not remove from {}: {}", shell_name, e));
                    }
                }
            }
        }
    }

    // 2. Remove ~/.xvnrc
    if let Ok(home) = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory")) {
        let config_path = home.join(".xvnrc");
        if config_path.exists() {
            fs::remove_file(&config_path)
                .context("Failed to remove ~/.xvnrc")?;
            removed_items.push("~/.xvnrc configuration".to_string());
        }
    }

    // 3. Remove ~/.xvn directory
    if let Ok(home) = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory")) {
        let xvn_dir = home.join(".xvn");
        if xvn_dir.exists() {
            fs::remove_dir_all(&xvn_dir)
                .context("Failed to remove ~/.xvn directory")?;
            removed_items.push("~/.xvn directory".to_string());
        }
    }

    // 4. Clear conflict warning
    InstallationDetector::clear_conflict();

    // Print success message
    println!();
    if removed_items.is_empty() {
        output::info("No xvn files found to remove.");
    } else {
        output::success("✓ Successfully removed:");
        for item in removed_items {
            output::info(&format!("  • {}", item));
        }
    }

    println!();

    // Show next steps for external installations
    if !installations.is_empty() {
        output::info("To complete uninstallation, remove external packages:");
        for (method, _) in &installations {
            output::info(&format!("  {}", method.uninstall_command().cyan()));
        }
        println!();
    }

    output::info("Please restart your shell or run:");
    output::info("  source ~/.bashrc  # or ~/.zshrc");
    println!();

    Ok(())
}
