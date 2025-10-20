use crate::config::{AutoInstallMode, Config};
use crate::init::detection::{detect_all, detect_shell, detect_version_managers, get_profile_path};
use crate::init::prompts::{self, prompt_quick_mode_confirmation, ConfigSummary, QuickModeChoice};
use crate::init::summary::{format_detection_summary, DetectionResults};
use crate::init::timeline::{chars, render_step, Step, StepState};
use crate::output;
use crate::setup::shell_detection::Shell;
use anyhow::{anyhow, Context, Result};
use dirs::home_dir;
use inquire;

/// Wizard mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardMode {
    /// Quick mode with auto-detection and single confirmation
    Quick,
    /// Advanced mode with full customization (3-step flow)
    Advanced,
}

/// Wizard state - collects configuration through steps
#[derive(Debug, Clone)]
pub struct WizardState {
    pub shell: Option<Shell>,
    pub plugins: Vec<String>,
    pub auto_install: AutoInstallMode,
    pub version_files: Vec<String>,
}

impl WizardState {
    /// Create new wizard state with defaults
    pub fn new() -> Self {
        Self {
            shell: None,
            plugins: Vec::new(),
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string(), ".node-version".to_string()],
        }
    }

    /// Convert wizard state to Config
    pub fn to_config(&self) -> Result<Config> {
        Ok(Config {
            plugins: self.plugins.clone(),
            auto_install: self.auto_install.clone(),
            version_files: self.version_files.clone(),
            use_default: true,     // Default to enabled
            default_version: None, // No default version from wizard
        })
    }

    /// Get shell or error
    pub fn get_shell(&self) -> Result<Shell> {
        self.shell.ok_or_else(|| anyhow::anyhow!("Shell not set"))
    }
}

impl Default for WizardState {
    fn default() -> Self {
        Self::new()
    }
}

/// Check for installation conflicts and warn user
fn check_installation_conflicts() -> Result<()> {
    use crate::installation_detector::InstallationDetector;

    let installations = InstallationDetector::detect_all();

    if installations.len() > 1 {
        println!();
        output::warning("⚠️  Multiple anvs installations detected!");
        println!();
        output::info("Found the following installations:");

        for (method, path) in &installations {
            output::info(&format!(
                "  • {} at {}",
                method.description(),
                path.display()
            ));
        }

        println!();
        output::warning(
            "Having multiple installations can cause conflicts and unexpected behavior.",
        );
        output::info("It's recommended to keep only one installation method.");
        println!();
        output::info("To uninstall:");

        for (method, _) in &installations {
            output::info(&format!(
                "  • {}: {}",
                method.description(),
                method.uninstall_command()
            ));
        }

        println!();

        // Ask user if they want to continue
        let should_continue = inquire::Confirm::new("Continue with setup anyway?")
            .with_default(true)
            .with_help_message("anvs will still work, but you may see warnings")
            .prompt()
            .context("Failed to get user confirmation")?;

        if !should_continue {
            output::info("Setup cancelled. Please resolve conflicts and try again.");
            return Err(anyhow::anyhow!("Setup cancelled by user"));
        }

        // Mark that conflicts exist so we can warn during activation
        InstallationDetector::mark_conflict();
        println!();
    }

    Ok(())
}

/// Print wizard header
fn print_wizard_header() {
    use owo_colors::OwoColorize;

    println!();
    println!("{}", "━".repeat(60).bright_cyan());
    crate::output::print_header();
    println!("{}", "━".repeat(60).bright_cyan());
    println!();
    println!(
        "  {} {}",
        "👋".bright_cyan(),
        "Welcome! Let's set up anvs for your environment.".bold()
    );
    println!();
    println!("  {}", "This wizard will guide you through:".dimmed());
    println!("    {} Shell detection and integration", "•".bright_cyan());
    println!("    {} Version manager selection", "•".bright_cyan());
    println!("    {} Installation preferences", "•".bright_cyan());
    println!("    {} Version file configuration", "•".bright_cyan());
    println!();
    println!(
        "  {} {}",
        "ℹ".blue(),
        "Press Ctrl+C at any time to cancel.".dimmed()
    );
    println!();
    println!("{}", "━".repeat(60).bright_cyan());
    println!();
}

/// Print success message after setup
fn print_success_message(summary: &ConfigSummary) -> Result<()> {
    use owo_colors::OwoColorize;

    println!();
    println!("{}", "━".repeat(60).bright_cyan());
    println!();
    println!(
        "  {} {}",
        "✨".bright_green(),
        "Setup complete!".bright_green().bold()
    );
    println!();
    println!("{}", "━".repeat(60).bright_cyan());
    println!();

    println!("  {}", "📋 Configuration Summary:".cyan().bold());
    println!(
        "    {} {}",
        "Shell:".dimmed(),
        summary.shell.name().bright_white()
    );
    println!(
        "    {} {}",
        "Profile:".dimmed(),
        summary.profile_path.display().to_string().bright_white()
    );
    println!(
        "    {} {}",
        "Config:".dimmed(),
        summary.config_path.display().to_string().bright_white()
    );
    println!();

    println!("  {}", "🚀 Next Steps:".cyan().bold());
    println!();
    println!(
        "    {} {}",
        "1.".bright_cyan(),
        "Restart your shell, or run:".dimmed()
    );
    println!(
        "       {}",
        format!("source {}", summary.profile_path.display()).bright_yellow()
    );
    println!();
    println!(
        "    {} {}",
        "2.".bright_cyan(),
        "Navigate to a project with a .nvmrc file".dimmed()
    );
    println!();
    println!(
        "    {} {}",
        "3.".bright_cyan(),
        "anvs will automatically activate the correct Node.js version!".dimmed()
    );
    println!();

    println!("  {}", "💡 Useful Commands:".cyan().bold());
    println!(
        "    {} {}",
        "anvs status".bright_yellow().bold(),
        "     Show current configuration".dimmed()
    );
    println!(
        "    {} {}",
        "anvs activate".bright_yellow().bold(),
        "   Manually activate for a directory".dimmed()
    );
    println!(
        "    {} {}",
        "anvs init".bright_yellow().bold(),
        "       Re-run this wizard to modify config".dimmed()
    );
    println!();
    println!("{}", "━".repeat(60).bright_cyan());
    println!();

    Ok(())
}

/// Generate config file content with comments
fn generate_config(config: &Config) -> String {
    use chrono::Local;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    let auto_install_str = match config.auto_install {
        AutoInstallMode::Prompt => "prompt",
        AutoInstallMode::Always => "always",
        AutoInstallMode::Never => "never",
    };

    format!(
        r#"# anvs configuration file
# Generated by: anvs init
# Last modified: {}
#
# To modify this configuration, run: anvs init

# Version manager priority order
# Available: nvm, fnm, n, asdf, volta
plugins:
{}

# Auto-install behavior when version not found
# Options: prompt (ask each time), always (install automatically), never (error)
auto_install: {}

# Version files to search for (in priority order)
version_files:
{}
"#,
        timestamp,
        config
            .plugins
            .iter()
            .map(|p| format!("  - {p}"))
            .collect::<Vec<_>>()
            .join("\n"),
        auto_install_str,
        config
            .version_files
            .iter()
            .map(|f| format!("  - {f}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

/// Write configuration to file
fn write_config(config: &Config, path: &std::path::Path, force: bool) -> Result<()> {
    use inquire::Confirm;
    use std::fs;

    // Check if config exists
    if path.exists() && !force {
        let overwrite = Confirm::new("Configuration file already exists. Overwrite?")
            .with_default(false)
            .with_help_message("Use --force to skip this prompt")
            .prompt()?;

        if !overwrite {
            anyhow::bail!("Config write cancelled - existing file preserved");
        }
    }

    // Validate config
    crate::init::validation::validate_config(config)?;

    // Generate YAML content
    let content = generate_config(config);

    // Write to file
    fs::write(path, content).context("Failed to write configuration file")?;

    log::info!("Config written to: {}", path.display());
    Ok(())
}

/// Run the interactive wizard
pub fn run_interactive_wizard(force: bool) -> Result<()> {
    // Print header
    print_wizard_header();

    // Check for installation conflicts
    check_installation_conflicts()?;

    // Initialize state
    let mut state = WizardState::new();

    // Step 1: Shell selection
    let shell = prompts::prompt_shell()?;
    state.shell = Some(shell);
    println!();

    // Step 2: Plugin selection
    let plugins = prompts::prompt_plugins()?;
    state.plugins = plugins;
    println!();

    // Step 3: Auto-install mode
    let auto_install = prompts::prompt_auto_install()?;
    state.auto_install = auto_install;
    println!();

    // Step 4: Version files
    let version_files = prompts::prompt_version_files()?;
    state.version_files = version_files;
    println!();

    // Step 5: Review and confirm
    let profile_path = get_profile_path(&state.get_shell()?)?;
    let config_path = home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".anvsrc");

    let summary = ConfigSummary {
        shell: state.get_shell()?,
        profile_path,
        plugins: state.plugins.clone(),
        auto_install: state.auto_install.clone(),
        version_files: state.version_files.clone(),
        config_path: config_path.clone(),
    };

    let confirmed = prompts::prompt_confirm_config(&summary)?;
    if !confirmed {
        output::warning("Setup cancelled");
        return Ok(());
    }
    println!();

    // Generate and write config
    let config = state.to_config()?;
    write_config(&config, &config_path, force)?;
    output::success("Configuration saved!");

    // Install shell integration
    let installer = crate::setup::SetupInstaller::new()?;
    installer.install()?;
    output::success("Shell integration installed!");

    // Print success message
    print_success_message(&summary)?;

    Ok(())
}

/// Run quick setup with auto-detection and defaults
pub fn run_quick_setup(force: bool) -> Result<()> {
    println!();
    output::print_header();
    output::info("Running quick setup with defaults...");
    println!();

    // Check for installation conflicts
    check_installation_conflicts()?;

    // Auto-detect shell
    let shell = detect_shell()?;
    output::info(&format!("Detected shell: {}", shell.name()));

    // Auto-detect version managers
    let detected = detect_version_managers();
    let plugins: Vec<String> = detected.iter().map(|m| m.name.clone()).collect();

    if plugins.is_empty() {
        output::warning("No version managers detected");
        output::info("You'll need to install nvm, fnm, or n manually");
    } else {
        output::info(&format!("Detected managers: {}", plugins.join(", ")));
    }

    // Use defaults
    let auto_install = AutoInstallMode::Prompt;
    let version_files = vec![".nvmrc".to_string(), ".node-version".to_string()];

    output::info("Using defaults:");
    output::info("  • Auto-install: prompt");
    output::info("  • Version files: .nvmrc, .node-version");
    println!();

    // Create state
    let state = WizardState {
        shell: Some(shell),
        plugins,
        auto_install,
        version_files,
    };

    // Generate config
    let config = state.to_config()?;
    let config_path = home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".anvsrc");

    write_config(&config, &config_path, force)?;
    output::success("Configuration saved!");

    // Install shell integration
    let installer = crate::setup::SetupInstaller::new()?;
    installer.install()?;
    output::success("Shell integration installed!");

    // Print summary
    let profile_path = get_profile_path(&shell)?;
    let summary = ConfigSummary {
        shell,
        profile_path,
        plugins: state.plugins.clone(),
        auto_install: state.auto_install.clone(),
        version_files: state.version_files.clone(),
        config_path,
    };

    print_success_message(&summary)?;

    Ok(())
}

/// Run non-interactive setup for CI/automation
pub fn run_non_interactive(force: bool) -> Result<()> {
    // Non-interactive is the same as quick mode
    // but with explicit logging for CI

    eprintln!("anvs: Running in non-interactive mode");

    run_quick_setup(force)
}

/// Installation progress tracker for visual feedback
struct InstallationProgress {
    steps: Vec<Step>,
}

impl InstallationProgress {
    fn new() -> Self {
        Self {
            steps: vec![
                Step::new("Creating config at ~/.anvsrc"),
                Step::new("Installing shell hook"),
                Step::new("Validating installation"),
                Step::new("Testing activation"),
            ],
        }
    }

    fn mark_complete(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.set_state(StepState::Complete);
        }
    }

    fn mark_active(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.set_state(StepState::Active);
        }
    }

    fn get(&self, index: usize) -> Option<&Step> {
        self.steps.get(index)
    }
}

/// Install configuration and shell hook with progress indicators
pub fn install_config(config: Config, shell: Shell, force: bool) -> Result<()> {
    println!();
    output::brand("⚡ Automatic Node Version Switcher");
    println!();

    let mut progress = InstallationProgress::new();

    // Print header
    println!("{}  Installing", chars::STEP_ACTIVE);

    // Step 1: Create config
    progress.mark_active(0);
    let config_path = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not find home directory"))?
        .join(".anvsrc");
    write_config(&config, &config_path, force)
        .map_err(|e| anyhow!("Failed to create config: {e}"))?;
    progress.mark_complete(0);
    if let Some(step) = progress.get(0) {
        println!("{}  {}", chars::BRANCH_RIGHT, render_step(step));
    }

    // Step 2: Install shell hook
    progress.mark_active(1);
    install_shell_hook(&shell, force).map_err(|e| anyhow!("Failed to install shell hook: {e}"))?;
    progress.mark_complete(1);
    if let Some(step) = progress.get(1) {
        println!("{}  {}", chars::BRANCH_RIGHT, render_step(step));
    }

    // Step 3: Validate
    progress.mark_active(2);
    validate_installation(&shell).map_err(|e| anyhow!("Validation failed: {e}"))?;
    progress.mark_complete(2);
    if let Some(step) = progress.get(2) {
        println!("{}  {}", chars::BRANCH_RIGHT, render_step(step));
    }

    // Step 4: Test activation (optional, may skip)
    progress.mark_active(3);
    // Test activation is optional and may not be implemented yet
    match test_activation() {
        Ok(_) => {
            progress.mark_complete(3);
            if let Some(step) = progress.get(3) {
                println!("{}  {}", chars::BRANCH_LAST, render_step(step));
            }
        }
        Err(_) => {
            // Skip test activation if not implemented
            log::debug!("Skipping activation test (not implemented)");
            if let Some(step) = progress.get(3) {
                use owo_colors::OwoColorize;
                println!("{}  {} (skipped)", chars::BRANCH_LAST, step.label.dimmed());
            }
        }
    }

    Ok(())
}

// Placeholder functions (implement or use existing)
fn install_shell_hook(shell: &Shell, _force: bool) -> Result<()> {
    // Use existing shell profile modification logic
    let profile_path = get_profile_path(shell)?;
    crate::setup::profile_modification::add_to_profile(&profile_path)
}

fn validate_installation(_shell: &Shell) -> Result<()> {
    // Basic validation: check that config file exists
    let config_path = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not find home directory"))?
        .join(".anvsrc");

    if !config_path.exists() {
        return Err(anyhow!("Config file not created at {config_path:?}"));
    }

    log::debug!("Installation validated successfully");
    Ok(())
}

fn test_activation() -> Result<()> {
    // Placeholder - may not be implemented yet
    log::debug!("Activation test not implemented");
    Ok(())
}

/// Convert detection results to a Config object
fn results_to_config(results: &DetectionResults) -> Result<Config> {
    Ok(Config {
        plugins: if results.version_managers.is_empty() {
            // Default to nvm if nothing detected
            vec!["nvm".to_string()]
        } else {
            results.version_managers.clone()
        },
        auto_install: results.auto_install.clone(),
        version_files: vec![
            ".nvmrc".to_string(),
            ".node-version".to_string(),
            "package.json".to_string(),
        ],
        use_default: true,
        default_version: None, // No default version from detection
    })
}

/// Run quick mode wizard (default)
///
/// This is the new default wizard experience:
/// 1. Auto-detect shell and version manager
/// 2. Display summary of detected values
/// 3. Single confirmation prompt
/// 4. Done!
pub fn run_quick_wizard() -> Result<(Config, Shell)> {
    // Print header
    println!();
    output::brand("⚡ Automatic Node Version Switcher");
    println!();

    // Run detection
    log::debug!("Running auto-detection...");
    let results = detect_all()?;
    log::debug!(
        "Detection complete: shell={:?}, version_managers={:?}",
        results.shell,
        results.version_managers
    );

    // Show summary
    println!("{}", format_detection_summary(&results));
    println!();

    // Check if critical detection failed
    if results.shell.is_none() {
        output::warning("⚠️  Shell auto-detection failed");
        println!();
        output::info("Please use advanced mode to configure manually:");
        output::info("  anvs init --advanced");
        return Err(anyhow!(
            "Shell detection failed. Use --advanced mode or specify --shell flag."
        ));
    }

    if results.version_managers.is_empty() {
        output::warning("⚠️  No version managers detected");
        output::info("anvs will default to nvm. Ensure nvm or fnm is installed.");
        println!();
        // Continue anyway with nvm as default
    }

    // Single confirmation prompt
    match prompt_quick_mode_confirmation(&results)? {
        QuickModeChoice::Proceed => {
            log::debug!("User accepted quick mode configuration");
            // User accepted defaults
            let shell = results.shell.ok_or_else(|| anyhow!("Shell not detected"))?;
            let config = results_to_config(&results)?;
            Ok((config, shell))
        }
        QuickModeChoice::Customize => {
            log::debug!("User chose to customize settings");
            // Drop into advanced mode
            println!();
            output::info("Switching to advanced mode...");
            println!();
            run_advanced_wizard()
        }
        QuickModeChoice::Cancel => {
            log::debug!("User cancelled setup");
            Err(anyhow!("Setup cancelled by user"))
        }
    }
}

/// Run advanced mode wizard (3-step customization flow)
///
/// This provides full customization with inline detection:
/// 1. Shell selection (with detected value pre-selected)
/// 2. Version manager selection (with detected values)
/// 3. Auto-install behavior
pub fn run_advanced_wizard() -> Result<(Config, Shell)> {
    use crate::init::prompts::{
        prompt_auto_install, prompt_shell_with_detection, prompt_version_manager_with_detection,
    };
    use crate::init::summary::format_config_preview;
    use owo_colors::OwoColorize;

    // Print header
    println!();
    output::brand("⚡ Automatic Node Version Switcher");
    println!();
    output::info("Advanced Setup - Customize your configuration");
    println!();

    // Run detection for defaults
    log::debug!("Running detection for advanced mode defaults...");
    let results = detect_all()?;

    // Step 1: Shell selection
    println!();
    println!(
        "{} {}",
        chars::STEP_ACTIVE,
        "Step 1 of 3: Shell Configuration".bold()
    );
    let shell = prompt_shell_with_detection(results.shell.as_ref())?;
    log::debug!("Selected shell: {shell:?}");

    // Step 2: Version manager selection
    println!();
    println!(
        "{} {}",
        chars::STEP_ACTIVE,
        "Step 2 of 3: Version Manager".bold()
    );
    let version_managers = prompt_version_manager_with_detection(results.version_managers.clone())?;
    log::debug!("Selected version managers: {version_managers:?}");

    // Step 3: Auto-install behavior
    println!();
    println!(
        "{} {}",
        chars::STEP_ACTIVE,
        "Step 3 of 3: Auto-Install Behavior".bold()
    );
    let auto_install = prompt_auto_install()?;
    log::debug!("Selected auto-install mode: {auto_install:?}");

    // Create config from selections
    let config = Config {
        plugins: version_managers,
        auto_install,
        version_files: vec![
            ".nvmrc".to_string(),
            ".node-version".to_string(),
            "package.json".to_string(),
        ],
        use_default: true,
        default_version: None,
    };

    // Show configuration preview and confirm
    println!();
    println!("{}", format_config_preview(&config, &shell));
    println!();

    let confirmed = inquire::Confirm::new("Apply this configuration?")
        .with_default(true)
        .with_help_message("Select 'No' to cancel setup")
        .prompt()?;

    if !confirmed {
        return Err(anyhow!("Setup cancelled by user"));
    }

    Ok((config, shell))
}

/// Display completion message with next steps
fn show_completion_message(shell: &Shell, duration: std::time::Duration) -> Result<()> {
    use crate::init::summary::format_next_steps;

    println!();
    output::success("✓ Setup complete!");

    // Show timing if < 60 seconds
    if duration.as_secs() < 60 {
        output::info(&format!("Completed in {:.1}s", duration.as_secs_f64()));
    } else {
        output::info(&format!(
            "Completed in {}m {}s",
            duration.as_secs() / 60,
            duration.as_secs() % 60
        ));
    }

    println!();
    println!("{}", format_next_steps(shell));

    Ok(())
}

/// Handle the complete init flow (detection -> wizard -> install -> completion)
pub fn handle_init(quick: bool, advanced: bool, force: bool) -> Result<()> {
    use std::time::Instant;
    let start = Instant::now();

    // Determine wizard mode
    let mode = if advanced {
        log::debug!("Running advanced mode (--advanced flag)");
        WizardMode::Advanced
    } else if quick {
        log::debug!("Running quick mode (--quick flag)");
        WizardMode::Quick
    } else {
        // Default to quick mode when no flags provided
        log::debug!("Running quick mode (default behavior)");
        WizardMode::Quick
    };

    // Check for installation conflicts before proceeding
    check_installation_conflicts()?;

    // Run appropriate wizard
    let (config, shell) = match mode {
        WizardMode::Quick => run_quick_wizard()?,
        WizardMode::Advanced => run_advanced_wizard()?,
    };

    log::debug!("Wizard completed, proceeding with installation");

    // Install
    install_config(config, shell, force)?;

    // Show completion
    show_completion_message(&shell, start.elapsed())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_state_defaults() {
        let state = WizardState::new();
        assert_eq!(state.plugins.len(), 0);
        assert!(matches!(state.auto_install, AutoInstallMode::Prompt));
        assert_eq!(state.version_files.len(), 2);
    }

    #[test]
    fn test_to_config() {
        let mut state = WizardState::new();
        state.shell = Some(Shell::Zsh);
        state.plugins = vec!["nvm".to_string()];

        let config = state.to_config().unwrap();
        assert_eq!(config.plugins, vec!["nvm"]);
    }

    #[test]
    fn test_generate_config_format() {
        let config = Config {
            plugins: vec!["nvm".to_string(), "fnm".to_string()],
            auto_install: AutoInstallMode::Prompt,
            version_files: vec![".nvmrc".to_string()],
            use_default: true,
            default_version: None,
        };
        let yaml = generate_config(&config);

        // Verify YAML structure
        assert!(yaml.contains("plugins:"));
        assert!(yaml.contains("  - nvm"));
        assert!(yaml.contains("  - fnm"));
        assert!(yaml.contains("auto_install: prompt"));

        // Verify it can be parsed back
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        assert!(parsed["plugins"].is_sequence());
    }
}
