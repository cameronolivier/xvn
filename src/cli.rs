use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

/// Automatic Node.js version switching for cd
#[derive(Parser, Debug)]
#[command(name = "anvs")]
#[command(
    about = "ANVS - Automatic Node Version Switcher for Node.js",
    long_about = r#"
anvs automatically switches your Node.js version when you cd into a directory
with a .nvmrc or .node-version file. When you leave a project directory, anvs
automatically returns to your default Node.js version.

After installation, run 'anvs init' to configure your shell with an interactive
wizard, or 'anvs init --quick' for automatic setup with sensible defaults.

Examples:
  anvs init               Interactive setup wizard (recommended)
  anvs init --quick       Quick setup with defaults
  anvs activate           Manually activate for current directory
  anvs status             Show configuration and test activation
  anvs set                Change configuration settings
  anvs uninstall          Completely remove anvs

For more information, visit: https://github.com/olvrcc/anvs
"#
)]
#[command(version)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize anvs with interactive configuration wizard
    ///
    /// This command guides you through initial setup with auto-detection
    /// and configuration of shell integration, version managers, and preferences.
    ///
    /// For quick setup with defaults: anvs init --quick
    /// For automation/CI: anvs init --non-interactive
    Init {
        /// Skip wizard and use sensible defaults
        #[arg(short, long)]
        quick: bool,

        /// Force overwrite existing configuration
        #[arg(short, long)]
        force: bool,

        /// Shell to configure (bash, zsh, or auto-detect)
        #[arg(short, long)]
        shell: Option<String>,

        /// Non-interactive mode for automation
        #[arg(long)]
        non_interactive: bool,
    },

    /// Set up shell integration (alias for 'init' for compatibility)
    ///
    /// This is an alias for 'anvs init' for backward compatibility.
    /// Use 'anvs init' for the full interactive wizard.
    #[clap(hide = true)] // Hide from main help but still works
    Setup {
        /// Shell to configure (bash, zsh, or auto-detect)
        #[arg(short, long, default_value = "auto")]
        shell: String,

        /// Force reinstallation even if already set up
        #[arg(short, long)]
        force: bool,
    },

    /// Manually activate Node.js version for a directory
    ///
    /// Normally anvs activates automatically on cd (after running setup).
    /// Use this command to manually activate for the current directory,
    /// or to test activation before setting up the shell hook.
    Activate {
        /// Directory to activate for (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Use default version if no version file found
        ///
        /// When enabled, anvs will switch to the version manager's default
        /// version (e.g., `nvm version default`) if no .nvmrc file is found.
        /// This is used internally by the shell hook when leaving project
        /// directories.
        #[arg(long)]
        use_default: bool,
    },

    /// Show configuration, installed plugins, and test activation
    ///
    /// Displays:
    /// - Active configuration settings
    /// - Available version managers (nvm, fnm, etc.)
    /// - Current directory's Node.js version (if any)
    /// - Activation timing for performance testing
    Status,

    /// Interactively change a specific configuration setting
    ///
    /// Easily update individual settings without re-running the full init wizard.
    ///
    /// Examples:
    ///   anvs set                    Choose setting from menu
    ///   anvs set auto-install       Change auto-install mode
    ///   anvs set plugins            Change version manager plugins
    ///   anvs set version-files      Change version file priority
    Set {
        /// Setting to change (auto-install, plugins, version-files)
        setting: Option<String>,
    },

    /// Uninstall anvs completely
    ///
    /// Removes all anvs installations, configuration files, and shell integration.
    /// This command detects all installation methods (npm, Homebrew, Cargo) and
    /// provides instructions for complete removal.
    ///
    /// WARNING: This will remove:
    ///   - ~/.anvs directory (all versions and binaries)
    ///   - ~/.anvsrc configuration file
    ///   - Shell integration from .bashrc/.zshrc
    ///   - All installed anvs packages (npm, Homebrew, Cargo)
    Uninstall {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Set log level based on verbose flag
    if cli.verbose {
        log::set_max_level(log::LevelFilter::Debug);
    }

    match cli.command {
        Some(Commands::Init {
            quick,
            force,
            shell,
            non_interactive,
        }) => {
            info!("Running init command (quick: {quick}, force: {force}, non_interactive: {non_interactive})");

            // TODO: Handle shell parameter when provided
            let _ = shell; // Silence unused warning for now

            crate::init::init(quick, non_interactive, force)
        }

        Some(Commands::Setup { shell, force }) => {
            // Redirect to init in quick mode
            info!("Running setup command (redirecting to init)");
            let _ = shell; // Silence unused warning for now

            crate::init::init(true, false, force)
        }
        Some(Commands::Activate { path, use_default }) => {
            info!("Running activate command for path: {path:?} (use_default: {use_default})");

            // Check for installation conflicts and show warning if flagged
            if crate::installation_detector::InstallationDetector::should_warn() {
                eprintln!();
                crate::output::warning("⚠️  Multiple anvs installations detected!");
                crate::output::info("Run 'anvs init' to see details and resolve conflicts.");
                eprintln!();
            }

            // Load config
            let config = crate::config::Config::load().context("failed to load configuration")?;

            // Create plugin registry
            let registry = crate::plugins::PluginRegistry::new(&config.plugins);

            // Open FD:3 for writing commands
            let mut fd3 = crate::shell::CommandWriter::new()?;

            // Create orchestrator
            let mut orchestrator =
                crate::activation::Orchestrator::new(&config, &registry, &mut fd3);

            // Run activation
            match orchestrator.activate(&path, use_default) {
                Ok(()) => Ok(()),
                Err(e) => {
                    // Print main error message
                    crate::output::error(&format!("{e}"));

                    // Print hint if available
                    if let Some(hint) = e.hint() {
                        eprintln!();
                        crate::output::info(&hint);
                    }

                    // Exit with error code
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Status) => {
            info!("Running status command");

            match crate::config::Config::load() {
                Ok(config) => {
                    crate::output::info(&format!("Plugins: {}", config.plugins.join(", ")));
                    crate::output::info(&format!("Auto-install: {:?}", config.auto_install));
                    crate::output::info(&format!(
                        "Version files: {}",
                        config.version_files.join(", ")
                    ));
                    crate::output::info(&format!(
                        "Use default version: {}",
                        if config.use_default {
                            "enabled"
                        } else {
                            "disabled"
                        }
                    ));

                    // Try to show the default version from available plugins
                    let registry = crate::plugins::PluginRegistry::new(&config.plugins);
                    for plugin in registry.plugins() {
                        if let Ok(true) = plugin.is_available() {
                            if let Ok(Some(default_version)) = plugin.default_version() {
                                crate::output::info(&format!(
                                    "Default version ({}): {}",
                                    plugin.name(),
                                    default_version
                                ));
                                break; // Only show first plugin's default
                            }
                        }
                    }
                }
                Err(e) => {
                    crate::output::error(&format!("Error loading config: {e}"));
                    std::process::exit(1);
                }
            }
            Ok(())
        }
        Some(Commands::Set { setting }) => {
            info!("Running set command for setting: {setting:?}");
            crate::commands::set::set_config(setting)
        }
        Some(Commands::Uninstall { force }) => {
            info!("Running uninstall command (force: {force})");
            crate::commands::uninstall::uninstall(force)
        }
        None => {
            // No subcommand provided - show help
            Cli::parse_from(["anvs", "--help"]);
            Ok(())
        }
    }
}
