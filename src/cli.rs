use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

/// Automatic Node.js version switching for cd
#[derive(Parser, Debug)]
#[command(name = "xvn")]
#[command(
    about = "Automatic Node.js version switching",
    long_about = r#"
xvn automatically switches your Node.js version when you cd into a directory
with a .nvmrc or .node-version file.

After installation, run 'xvn init' to configure your shell with an interactive
wizard, or 'xvn init --quick' for automatic setup with sensible defaults.

Examples:
  xvn init               Interactive setup wizard (recommended)
  xvn init --quick       Quick setup with defaults
  xvn activate           Manually activate for current directory
  xvn status             Show configuration and test activation

For more information, visit: https://github.com/cameronolivier/xvn
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
    /// Initialize xvn with interactive configuration wizard
    ///
    /// This command guides you through initial setup with auto-detection
    /// and configuration of shell integration, version managers, and preferences.
    ///
    /// For quick setup with defaults: xvn init --quick
    /// For automation/CI: xvn init --non-interactive
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
    /// This is an alias for 'xvn init' for backward compatibility.
    /// Use 'xvn init' for the full interactive wizard.
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
    /// Normally xvn activates automatically on cd (after running setup).
    /// Use this command to manually activate for the current directory,
    /// or to test activation before setting up the shell hook.
    Activate {
        /// Directory to activate for (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Show configuration, installed plugins, and test activation
    ///
    /// Displays:
    /// - Active configuration settings
    /// - Available version managers (nvm, fnm, etc.)
    /// - Current directory's Node.js version (if any)
    /// - Activation timing for performance testing
    Status,
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
        Some(Commands::Activate { path }) => {
            info!("Running activate command for path: {path:?}");

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
            match orchestrator.activate(&path) {
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
                }
                Err(e) => {
                    crate::output::error(&format!("Error loading config: {e}"));
                    std::process::exit(1);
                }
            }
            Ok(())
        }
        None => {
            // No subcommand provided - show help
            Cli::parse_from(["xvn", "--help"]);
            Ok(())
        }
    }
}
