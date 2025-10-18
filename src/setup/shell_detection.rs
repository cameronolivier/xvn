use anyhow::{Context, Result};
use log::debug;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
}

impl Shell {
    /// Detects the current shell from environment variables
    pub fn detect() -> Result<Self> {
        // Try $SHELL first
        if let Ok(shell_path) = env::var("SHELL") {
            debug!("Detected shell from $SHELL: {shell_path}");
            return Self::from_path(&shell_path);
        }

        // Fallback to parent process (not implemented in MVP)
        anyhow::bail!("Could not detect shell. Please set $SHELL environment variable.");
    }

    /// Determines shell from path
    fn from_path(path: &str) -> Result<Self> {
        let shell_name = Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .context("Invalid shell path")?;

        match shell_name {
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            _ => anyhow::bail!(
                "Unsupported shell: {shell_name}. anvs currently supports bash and zsh."
            ),
        }
    }

    /// Returns the name of this shell
    pub fn name(&self) -> &str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
        }
    }

    /// Returns the profile files for this shell in priority order
    ///
    /// The first existing file will be used, or the first in the list
    /// if none exist.
    pub fn profile_files(&self, home: &Path) -> Vec<PathBuf> {
        match self {
            Self::Bash => vec![
                home.join(".bashrc"),
                home.join(".bash_profile"),
                home.join(".profile"),
            ],
            Self::Zsh => vec![home.join(".zshrc"), home.join(".zprofile")],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path() {
        assert_eq!(Shell::from_path("/bin/bash").unwrap(), Shell::Bash);
        assert_eq!(Shell::from_path("/usr/local/bin/zsh").unwrap(), Shell::Zsh);
        assert!(Shell::from_path("/bin/fish").is_err());
    }

    #[test]
    fn test_shell_name() {
        assert_eq!(Shell::Bash.name(), "bash");
        assert_eq!(Shell::Zsh.name(), "zsh");
    }
}
