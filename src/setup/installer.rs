use anyhow::{Context, Result};
use dirs::home_dir;
use log::{debug, info, warn};
use std::path::PathBuf;

use super::profile_modification;
use super::shell_detection::Shell;

pub struct SetupInstaller {
    home: PathBuf,
    shell: Shell,
}

impl SetupInstaller {
    pub fn new() -> Result<Self> {
        let home = home_dir().context("Could not determine home directory")?;
        let shell = Shell::detect()?;

        debug!("Detected shell: {}", shell.name());
        debug!("Home directory: {}", home.display());

        Ok(Self { home, shell })
    }

    fn find_profile(&self) -> Result<PathBuf> {
        let candidates = self.shell.profile_files(&self.home);

        for candidate in &candidates {
            if candidate.exists() {
                debug!("Found existing profile: {}", candidate.display());
                return Ok(candidate.clone());
            }
        }

        let default = candidates
            .first()
            .context("No profile candidates found")?
            .clone();

        warn!(
            "No existing profile found, will create: {}",
            default.display()
        );
        Ok(default)
    }

    /// Installs or updates xvn shell integration.
    pub fn install(&self) -> Result<()> {
        info!("Setting up xvn shell integration for {}", self.shell.name());

        let profile = self.find_profile()?;
        profile_modification::add_to_profile(&profile)?;

        Ok(())
    }

    pub fn print_instructions(&self) -> Result<()> {
        let profile = self.find_profile()?;

        println!();
        crate::output::print_header();
        crate::output::success("setup complete!");
        println!();
        crate::output::info("Your shell profile has been updated.");
        println!("  Shell:   {}", self.shell.name());
        println!("  Profile: {}", profile.display());
        println!();
        crate::output::info("To start using xvn:");
        println!("  1. Restart your shell, or run:");
        println!("       source {}", profile.display());
        println!("  2. Navigate to a project with a .nvmrc file");
        println!();
        crate::output::info("For more information:");
        println!("  xvn --help");
        println!();

        Ok(())
    }
}

impl Default for SetupInstaller {
    fn default() -> Self {
        Self::new().expect("Failed to create SetupInstaller")
    }
}
