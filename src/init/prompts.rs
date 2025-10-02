use crate::setup::shell_detection::Shell;
use crate::init::detection::{detect_shell, get_profile_path, detect_version_managers};
use crate::config::AutoInstallMode;
use crate::output;
use anyhow::Result;
use inquire::{Confirm, Select, MultiSelect};
use std::path::PathBuf;

/// Prompt user to select shell
pub fn prompt_shell() -> Result<Shell> {
    output::info("Step 1/5: Shell Detection");
    println!();

    // Try to detect shell
    let detected = detect_shell()?;
    let profile_path = get_profile_path(&detected)?;

    println!("  Detected shell: {}", detected.name());
    println!("  Profile: {}", profile_path.display());
    println!();

    // Ask for confirmation
    let use_detected = Confirm::new(&format!("Use {}?", detected.name()))
        .with_default(true)
        .with_help_message("Press Enter to confirm, or 'n' to select manually")
        .prompt()?;

    if use_detected {
        output::success(&format!("Using {}", detected.name()));
        return Ok(detected);
    }

    // Manual selection
    let shell_names = vec!["bash", "zsh"];

    let selected = Select::new("Select your shell:", shell_names)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt()?;

    let shell = match selected {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        _ => Shell::Bash, // Default fallback
    };

    output::success(&format!("Using {}", shell.name()));
    Ok(shell)
}

/// Prompt user to select version managers
pub fn prompt_plugins() -> Result<Vec<String>> {
    output::info("Step 2/5: Version Managers");
    println!();

    // Detect installed managers
    let detected = detect_version_managers();

    if detected.is_empty() {
        output::warning("No version managers detected!");
        println!();
        println!("  xvn requires a version manager to be installed:");
        println!("  • nvm: https://github.com/nvm-sh/nvm");
        println!("  • fnm: https://github.com/Schniz/fnm");
        println!("  • n: https://github.com/tj/n");
        println!();

        let proceed = Confirm::new("Continue setup anyway?")
            .with_default(false)
            .with_help_message("You can install a version manager later")
            .prompt()?;

        if !proceed {
            anyhow::bail!("Setup cancelled - please install a version manager first");
        }

        // Return empty list but allow setup to continue
        return Ok(Vec::new());
    }

    // Display detected managers
    println!("  Detected version managers:");
    for manager in &detected {
        let path_str = manager.path.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "unknown path".to_string());
        println!("  ✓ {} ({})", manager.name, path_str);
    }
    println!();

    // Create options for MultiSelect
    let options: Vec<String> = detected.iter()
        .map(|m| format!("{} ({})",
            m.name,
            m.path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "system".to_string())
        ))
        .collect();

    // Pre-select all detected (using indices)
    let defaults: Vec<usize> = (0..options.len()).collect();

    let selected = MultiSelect::new("Select version managers to use:", options.clone())
        .with_default(&defaults)
        .with_help_message("Space to toggle, Enter to confirm")
        .prompt()?;

    // Extract manager names from selections
    let mut plugins: Vec<String> = selected.iter()
        .filter_map(|sel| {
            // Parse "nvm (path)" -> "nvm"
            sel.split_whitespace().next().map(|s| s.to_string())
        })
        .collect();

    if plugins.is_empty() {
        output::warning("No version managers selected");
        plugins = detected.iter()
            .map(|m| m.name.clone())
            .collect();
        output::info(&format!("Defaulting to: {}", plugins.join(", ")));
    } else {
        output::success(&format!("Selected: {}", plugins.join(", ")));
    }

    // TODO: Prompt for priority order if multiple selected
    // For now, keep detection order

    Ok(plugins)
}

/// Prompt user for auto-install preference
pub fn prompt_auto_install() -> Result<AutoInstallMode> {
    output::info("Step 3/5: Auto-Install Behavior");
    println!();

    println!("  When a required Node.js version isn't installed:");
    println!();

    let options = vec![
        "Prompt - Ask before installing (recommended)",
        "Always - Install automatically without asking",
        "Never - Show error, don't install",
    ];

    let selected = Select::new("Choose auto-install mode:", options)
        .with_help_message("Use arrow keys and Enter to select")
        .prompt()?;

    let mode = match selected {
        s if s.starts_with("Prompt") => AutoInstallMode::Prompt,
        s if s.starts_with("Always") => AutoInstallMode::Always,
        s if s.starts_with("Never") => AutoInstallMode::Never,
        _ => AutoInstallMode::Prompt, // Default fallback
    };

    let mode_str = match mode {
        AutoInstallMode::Prompt => "prompt",
        AutoInstallMode::Always => "always",
        AutoInstallMode::Never => "never",
    };

    output::success(&format!("Auto-install: {}", mode_str));
    Ok(mode)
}

/// Prompt user for version file preferences
pub fn prompt_version_files() -> Result<Vec<String>> {
    output::info("Step 4/5: Version Files");
    println!();

    println!("  Which files should xvn check for version information?");
    println!();

    let options = vec![
        ".nvmrc (standard Node.js convention)",
        ".node-version (alternative format)",
        ".tool-versions (asdf compatibility)",
    ];

    // Default to .nvmrc and .node-version (using indices)
    let defaults = vec![0, 1]; // First two options

    let selected = MultiSelect::new("Select version files:", options)
        .with_default(&defaults)
        .with_help_message("Space to toggle, Enter to confirm")
        .prompt()?;

    // Extract filenames
    let files: Vec<String> = selected.iter()
        .map(|s| {
            // Parse ".nvmrc (description)" -> ".nvmrc"
            s.split_whitespace()
                .next()
                .unwrap_or(".nvmrc")
                .to_string()
        })
        .collect();

    if files.is_empty() {
        output::warning("No version files selected, using defaults");
        return Ok(vec![".nvmrc".to_string(), ".node-version".to_string()]);
    }

    output::success(&format!("Version files: {}", files.join(", ")));
    Ok(files)
}

pub struct ConfigSummary {
    pub shell: Shell,
    pub profile_path: PathBuf,
    pub plugins: Vec<String>,
    pub auto_install: AutoInstallMode,
    pub version_files: Vec<String>,
    pub config_path: PathBuf,
}

/// Prompt user to review and confirm configuration
pub fn prompt_confirm_config(summary: &ConfigSummary) -> Result<bool> {
    output::info("Step 5/5: Review Configuration");
    println!();

    println!("  {:<16} {}", "Shell:", summary.shell.name());
    println!("  {:<16} {}", "Profile:", summary.profile_path.display());
    println!("  {:<16} {}", "Plugins:", summary.plugins.join(", "));

    let auto_install_str = match summary.auto_install {
        AutoInstallMode::Prompt => "prompt",
        AutoInstallMode::Always => "always",
        AutoInstallMode::Never => "never",
    };
    println!("  {:<16} {}", "Auto-install:", auto_install_str);
    println!("  {:<16} {}", "Version files:", summary.version_files.join(", "));
    println!("  {:<16} {}", "Config file:", summary.config_path.display());
    println!();

    let confirmed = Confirm::new("Looks good?")
        .with_default(true)
        .with_help_message("Press Enter to confirm, or 'n' to cancel")
        .prompt()?;

    Ok(confirmed)
}

#[cfg(test)]
mod tests {
    // Note: Can't easily unit test interactive prompts
    // These will be tested manually with: cargo run -- init
}
