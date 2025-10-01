use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

/// Extreme Version Switcher for Node.js
#[derive(Parser, Debug)]
#[command(name = "xvn")]
#[command(about = "Automatic Node.js version switching", long_about = None)]
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
    /// Install xvn shell hooks and create default config
    Setup {
        /// Shell to configure (bash, zsh, or auto-detect)
        #[arg(short, long, default_value = "auto")]
        shell: String,

        /// Force reinstallation even if already set up
        #[arg(short, long)]
        force: bool,
    },

    /// Activate Node.js version for current directory
    Activate {
        /// Directory to activate for (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Show current xvn status and configuration
    Status,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Set log level based on verbose flag
    if cli.verbose {
        log::set_max_level(log::LevelFilter::Debug);
    }

    match cli.command {
        Some(Commands::Setup { shell, force }) => {
            info!("Running setup command (shell: {shell}, force: {force})");

            let installer = crate::setup::SetupInstaller::new()?;

            // Check if already installed (unless force flag is set)
            if !force && installer.is_installed()? {
                println!("xvn is already installed.");
                println!("Run 'xvn status' to verify your installation.");
                println!("Use --force to reinstall.");
                return Ok(());
            }

            installer.install()?;
            installer.print_instructions()?;

            Ok(())
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
                    eprintln!("Error: {}", e);

                    // Print hint if available
                    if let Some(hint) = e.hint() {
                        eprintln!();
                        eprintln!("{}", hint);
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
                    println!("xvn status:");
                    println!("  Plugins: {}", config.plugins.join(", "));
                    println!("  Auto-install: {:?}", config.auto_install);
                    println!("  Version files: {}", config.version_files.join(", "));
                }
                Err(e) => {
                    eprintln!("Error loading config: {e}");
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
