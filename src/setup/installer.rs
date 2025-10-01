use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use log::{debug, info, warn};
use dirs::home_dir;

use super::shell_detection::Shell;
use super::profile_modification;

const XVN_SH_CONTENT: &str = include_str!("../../shell/xvn.sh");

pub struct SetupInstaller {
    home: PathBuf,
    shell: Shell,
}

impl SetupInstaller {
    /// Creates a new SetupInstaller
    pub fn new() -> Result<Self> {
        let home = home_dir().context("Could not determine home directory")?;
        let shell = Shell::detect()?;

        debug!("Detected shell: {}", shell.name());
        debug!("Home directory: {}", home.display());

        Ok(Self { home, shell })
    }

    /// Returns the xvn installation directory (~/.xvn)
    fn xvn_dir(&self) -> PathBuf {
        self.home.join(".xvn")
    }

    /// Returns the xvn bin directory (~/.xvn/bin)
    fn xvn_bin_dir(&self) -> PathBuf {
        self.xvn_dir().join("bin")
    }

    /// Returns the path where xvn.sh will be installed
    fn xvn_sh_path(&self) -> PathBuf {
        self.xvn_bin_dir().join("xvn.sh")
    }

    /// Returns the path to the default config file
    fn config_path(&self) -> PathBuf {
        self.home.join(".xvnrc")
    }

    /// Finds the best profile file to modify
    fn find_profile(&self) -> Result<PathBuf> {
        let candidates = self.shell.profile_files(&self.home);

        // Use first existing file
        for candidate in &candidates {
            if candidate.exists() {
                debug!("Found existing profile: {}", candidate.display());
                return Ok(candidate.clone());
            }
        }

        // No existing file, use first candidate
        let default = candidates.first()
            .context("No profile candidates found")?
            .clone();

        warn!("No existing profile found, will create: {}", default.display());
        Ok(default)
    }

    /// Checks if xvn is already installed
    pub fn is_installed(&self) -> Result<bool> {
        let xvn_sh = self.xvn_sh_path();
        if !xvn_sh.exists() {
            return Ok(false);
        }

        let profile = self.find_profile()?;
        profile_modification::is_already_installed(&profile)
    }

    /// Installs xvn shell integration
    pub fn install(&self) -> Result<()> {
        info!("Installing xvn shell integration for {}", self.shell.name());

        // Create directories
        let bin_dir = self.xvn_bin_dir();
        fs::create_dir_all(&bin_dir)
            .with_context(|| format!("Failed to create directory: {}", bin_dir.display()))?;
        info!("Created directory: {}", bin_dir.display());

        // Copy xvn.sh
        let xvn_sh = self.xvn_sh_path();
        fs::write(&xvn_sh, XVN_SH_CONTENT)
            .with_context(|| format!("Failed to write xvn.sh: {}", xvn_sh.display()))?;
        info!("Installed xvn.sh: {}", xvn_sh.display());

        // Modify profile
        let profile = self.find_profile()?;
        profile_modification::add_to_profile(&profile, &xvn_sh)?;

        // Create default config if it doesn't exist
        let config = self.config_path();
        if !config.exists() {
            self.create_default_config()?;
        } else {
            info!("Config file already exists: {}", config.display());
        }

        Ok(())
    }

    /// Creates a default ~/.xvnrc config file
    fn create_default_config(&self) -> Result<()> {
        let config = self.config_path();

        let default_config = r#"# xvn configuration file
# See https://github.com/cameronolivier/xvn for documentation

# Version files to search for (in priority order)
version_files:
  - .nvmrc
  - .node-version

# Plugin priority order
plugins:
  - nvm
  - fnm

# Auto-install mode: prompt (default), always, never
auto_install: prompt
"#;

        fs::write(&config, default_config)
            .with_context(|| format!("Failed to write config: {}", config.display()))?;

        info!("Created default config: {}", config.display());
        Ok(())
    }

    /// Prints setup instructions
    pub fn print_instructions(&self) -> Result<()> {
        let profile = self.find_profile()?;

        println!("\n xvn installation complete!\n");
        println!("Configuration:");
        println!("  Shell:   {}", self.shell.name());
        println!("  Profile: {}", profile.display());
        println!("  Config:  {}", self.config_path().display());
        println!("\nTo start using xvn:");
        println!("  1. Restart your shell, or run:");
        println!("       source {}", profile.display());
        println!("  2. Navigate to a project with a .nvmrc file");
        println!("  3. xvn will automatically activate the correct Node.js version");
        println!("\nDebug mode (if you encounter issues):");
        println!("  export XVN_DEBUG=1");
        println!("\nFor more information:");
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
