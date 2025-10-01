use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use log::info;

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
            info!("Running setup command (shell: {}, force: {})", shell, force);
            println!("Setup command - not yet implemented");
            println!("  Shell: {}", shell);
            println!("  Force: {}", force);
            Ok(())
        }
        Some(Commands::Activate { path }) => {
            info!("Running activate command for path: {:?}", path);
            println!("Activate command - not yet implemented");
            println!("  Path: {}", path.display());
            Ok(())
        }
        Some(Commands::Status) => {
            info!("Running status command");
            println!("Status command - not yet implemented");
            Ok(())
        }
        None => {
            // No subcommand provided - show help
            Cli::parse_from(&["xvn", "--help"]);
            Ok(())
        }
    }
}
