//! Interactive configuration setter
//!
//! Provides `xvn set` command for easily updating individual config settings

use anyhow::{Context, Result};
use inquire::Select;
use std::fs;
use std::path::PathBuf;

use crate::config::{AutoInstallMode, Config};
use crate::output;

/// Set a specific configuration value interactively
pub fn set_config(setting: Option<String>) -> Result<()> {
    // Load current config
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let config_file = home.join(".xvnrc");
    let mut config = if config_file.exists() {
        let content = fs::read_to_string(&config_file).context("Failed to read config file")?;
        serde_yaml::from_str::<Config>(&content).context("Failed to parse config file")?
    } else {
        output::warning("No config file found. Please run 'xvn init' first.");
        return Ok(());
    };

    // If no setting specified, show menu
    let setting = match setting {
        Some(s) => s,
        None => {
            let options = vec!["auto-install", "plugins", "version-files", "use-default"];

            Select::new("Which setting would you like to change?", options)
                .prompt()?
                .to_string()
        }
    };

    // Handle the setting
    match setting.as_str() {
        "auto-install" => set_auto_install(&mut config)?,
        "plugins" => set_plugins(&mut config)?,
        "version-files" => set_version_files(&mut config)?,
        "use-default" => set_use_default(&mut config)?,
        _ => {
            output::error(&format!("Unknown setting: {setting}"));
            output::info("Available settings: auto-install, plugins, version-files, use-default");
            return Ok(());
        }
    }

    // Save config
    save_config(&config, &config_file)?;

    output::success(&format!("Configuration updated: {setting}"));
    output::info(&format!("Config saved to: {}", config_file.display()));

    Ok(())
}

fn set_auto_install(config: &mut Config) -> Result<()> {
    use owo_colors::OwoColorize;

    println!();
    println!("  {} {}", "⚙️".cyan(), "Auto-Install Setting".bold());
    println!();
    println!(
        "  Current: {}",
        format!("{:?}", config.auto_install).yellow()
    );
    println!();

    let options = [
        ("prompt", "Ask each time (recommended)"),
        ("always", "Install automatically"),
        ("never", "Never install, show error"),
    ];

    let choices: Vec<String> = options
        .iter()
        .map(|(name, desc)| format!("{} - {}", name.cyan(), desc))
        .collect();

    let selection = Select::new("Select auto-install mode:", choices)
        .with_help_message("↑↓ to navigate, Enter to select")
        .prompt()?;

    // Extract the mode from selection
    let mode = if selection.starts_with("prompt") {
        AutoInstallMode::Prompt
    } else if selection.starts_with("always") {
        AutoInstallMode::Always
    } else {
        AutoInstallMode::Never
    };

    config.auto_install = mode;

    Ok(())
}

fn set_plugins(config: &mut Config) -> Result<()> {
    use inquire::MultiSelect;
    use owo_colors::OwoColorize;

    println!();
    println!("  {} {}", "🔌".cyan(), "Version Manager Plugins".bold());
    println!();
    println!("  Current: {}", config.plugins.join(", ").yellow());
    println!();

    let all_plugins = [
        ("nvm", "Node Version Manager"),
        ("fnm", "Fast Node Manager"),
        ("n", "Node version management"),
        ("asdf", "Extendable version manager"),
        ("volta", "JavaScript toolchain manager"),
    ];

    let options: Vec<String> = all_plugins
        .iter()
        .map(|(name, desc)| format!("{} - {}", name.cyan(), desc))
        .collect();

    // Pre-select currently configured plugins
    let defaults: Vec<usize> = config
        .plugins
        .iter()
        .filter_map(|p| all_plugins.iter().position(|(name, _)| name == &p.as_str()))
        .collect();

    let selections = MultiSelect::new("Select version managers (in priority order):", options)
        .with_help_message("↑↓ to navigate, Space to select, Enter to confirm")
        .with_default(&defaults)
        .prompt()?;

    // Extract plugin names from selections
    let new_plugins: Vec<String> = selections
        .iter()
        .filter_map(|s| {
            let name = s.split_whitespace().next()?;
            Some(name.to_string())
        })
        .collect();

    if new_plugins.is_empty() {
        output::warning("No plugins selected. Config unchanged.");
        return Ok(());
    }

    config.plugins = new_plugins;

    Ok(())
}

fn set_version_files(config: &mut Config) -> Result<()> {
    use inquire::MultiSelect;
    use owo_colors::OwoColorize;

    println!();
    println!("  {} {}", "📄".cyan(), "Version Files".bold());
    println!();
    println!("  Current: {}", config.version_files.join(", ").yellow());
    println!();

    let all_files = [
        (".nvmrc", "nvm version file"),
        (".node-version", "Universal Node version file"),
        ("package.json", "npm package.json engines.node"),
    ];

    let options: Vec<String> = all_files
        .iter()
        .map(|(name, desc)| format!("{} - {}", name.cyan(), desc))
        .collect();

    // Pre-select currently configured files
    let defaults: Vec<usize> = config
        .version_files
        .iter()
        .filter_map(|f| all_files.iter().position(|(name, _)| name == &f.as_str()))
        .collect();

    let selections = MultiSelect::new("Select version files (in priority order):", options)
        .with_help_message("↑↓ to navigate, Space to select, Enter to confirm")
        .with_default(&defaults)
        .prompt()?;

    // Extract file names from selections
    let new_files: Vec<String> = selections
        .iter()
        .filter_map(|s| {
            let name = s.split_whitespace().next()?;
            Some(name.to_string())
        })
        .collect();

    if new_files.is_empty() {
        output::warning("No version files selected. Config unchanged.");
        return Ok(());
    }

    config.version_files = new_files;

    Ok(())
}

fn set_use_default(config: &mut Config) -> Result<()> {
    use inquire::Confirm;
    use owo_colors::OwoColorize;

    println!();
    println!("  {} {}", "🔄".cyan(), "Use Default Version".bold());
    println!();
    let status_text = if config.use_default {
        "enabled".green().to_string()
    } else {
        "disabled".red().to_string()
    };
    println!("  Current: {status_text}");
    println!();
    println!("  When enabled, xvn automatically switches to your version manager's");
    println!("  default Node.js version when you leave a project directory.");
    println!();
    println!("  For nvm: Uses the version aliased as 'default' (nvm alias default)");
    println!("  For fnm: Uses the fnm default version");
    println!();

    let enable = Confirm::new("Enable automatic switch to default version?")
        .with_default(config.use_default)
        .with_help_message("Recommended: yes")
        .prompt()?;

    config.use_default = enable;

    Ok(())
}

fn save_config(config: &Config, path: &PathBuf) -> Result<()> {
    use chrono::Local;

    // Generate YAML with comments
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    let content = format!(
        "# xvn configuration file\n\
         # Generated by: xvn set\n\
         # Last modified: {}\n\
         #\n\
         # To modify this configuration, run: xvn set <setting>\n\
         \n\
         # Version manager priority order\n\
         # Available: nvm, fnm, n, asdf, volta\n\
         plugins:\n{}\n\
         # Auto-install behavior when version not found\n\
         # Options: prompt (ask each time), always (install automatically), never (error)\n\
         auto_install: {}\n\
         \n\
         # Version files to search for (in priority order)\n\
         version_files:\n{}\n\
         # Automatically switch to default version when leaving project directories\n\
         # When enabled, xvn switches to your version manager's default Node.js version\n\
         # (e.g., 'nvm alias default') when you cd out of a project directory.\n\
         use_default: {}\n",
        timestamp,
        config
            .plugins
            .iter()
            .map(|p| format!("  - {p}"))
            .collect::<Vec<_>>()
            .join("\n"),
        match config.auto_install {
            AutoInstallMode::Prompt => "prompt",
            AutoInstallMode::Always => "always",
            AutoInstallMode::Never => "never",
        },
        config
            .version_files
            .iter()
            .map(|f| format!("  - {f}"))
            .collect::<Vec<_>>()
            .join("\n"),
        config.use_default,
    );

    fs::write(path, content).context("Failed to write config file")?;

    Ok(())
}
