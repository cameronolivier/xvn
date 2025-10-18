//! Uninstall command - removes all anvs installations and configuration

use crate::installation_detector::InstallationDetector;
use crate::output;
use crate::setup::profile_modification;
use anyhow::{Context, Result};
use inquire::Confirm;
use owo_colors::OwoColorize;
use std::fs;

/// Uninstall anvs completely
pub fn uninstall(force: bool) -> Result<()> {
    println!();
    output::warning("⚠️  ANVS Uninstall");
    println!();

    // Detect all installations
    let installations = InstallationDetector::detect_all();

    if installations.is_empty() {
        output::info("No external anvs installations detected.");
        println!();
    } else {
        output::info("Detected anvs installations:");
        for (method, path) in &installations {
            output::info(&format!(
                "  • {} at {}",
                method.description(),
                path.display()
            ));
        }
        println!();
    }

    // Show what will be removed
    output::info("This will remove:");
    output::info("  • ~/.anvs directory (all versions and binaries)");
    output::info("  • ~/.anvsrc configuration file");
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
        let confirmed = Confirm::new("Are you sure you want to uninstall anvs?")
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
    if let Ok(home) =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
    {
        let shells = vec![(home.join(".bashrc"), "bash"), (home.join(".zshrc"), "zsh")];

        for (profile_path, shell_name) in shells {
            if profile_path.exists() {
                match profile_modification::remove_from_profile(&profile_path) {
                    Ok(true) => {
                        removed_items.push(format!("Shell integration from {shell_name}"));
                    }
                    Ok(false) => {
                        // Not present, skip
                    }
                    Err(e) => {
                        output::warning(&format!(
                            "Warning: Could not remove from {shell_name}: {e}"
                        ));
                    }
                }
            }
        }
    }

    // 2. Remove ~/.anvsrc
    if let Ok(home) =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
    {
        let config_path = home.join(".anvsrc");
        if config_path.exists() {
            fs::remove_file(&config_path).context("Failed to remove ~/.anvsrc")?;
            removed_items.push("~/.anvsrc configuration".to_string());
        }
    }

    // 3. Remove ~/.anvs directory
    if let Ok(home) =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
    {
        let anvs_dir = home.join(".anvs");
        if anvs_dir.exists() {
            fs::remove_dir_all(&anvs_dir).context("Failed to remove ~/.anvs directory")?;
            removed_items.push("~/.anvs directory".to_string());
        }
    }

    // 4. Clear conflict warning
    InstallationDetector::clear_conflict();

    // Print success message
    println!();
    if removed_items.is_empty() {
        output::info("No anvs files found to remove.");
    } else {
        output::success("✓ Successfully removed:");
        for item in removed_items {
            output::info(&format!("  • {item}"));
        }
    }

    println!();

    // Handle external installations
    if !installations.is_empty() {
        println!();
        output::info("Detected external installations:");
        for (method, _) in &installations {
            output::info(&format!("  • {}", method.description()));
        }
        println!();

        let should_uninstall = if force {
            true
        } else {
            Confirm::new("Also uninstall external packages?")
                .with_default(true)
                .with_help_message("This will run the uninstall commands for detected packages")
                .prompt()
                .context("Failed to get confirmation")?
        };

        if should_uninstall {
            println!();
            output::info("Uninstalling external packages...");
            println!();

            for (method, _) in &installations {
                let cmd = method.uninstall_command();
                output::info(&format!("Running: {}", cmd.cyan()));

                // Execute the uninstall command
                let result = std::process::Command::new("sh").arg("-c").arg(cmd).status();

                match result {
                    Ok(status) if status.success() => {
                        output::success(&format!("✓ Uninstalled {}", method.description()));
                    }
                    Ok(status) => {
                        output::warning(&format!(
                            "Warning: {} returned exit code {}",
                            method.description(),
                            status.code().unwrap_or(-1)
                        ));
                        output::info("You may need to run the command manually");
                    }
                    Err(e) => {
                        output::warning(&format!(
                            "Warning: Failed to uninstall {}: {}",
                            method.description(),
                            e
                        ));
                        output::info(&format!("Please run manually: {cmd}"));
                    }
                }
            }
            println!();
        } else {
            println!();
            output::info("To complete uninstallation manually, run:");
            for (method, _) in &installations {
                output::info(&format!("  {}", method.uninstall_command().cyan()));
            }
            println!();
        }
    }

    output::info("Please restart your shell or run:");
    output::info("  source ~/.bashrc  # or ~/.zshrc");
    println!();

    Ok(())
}
