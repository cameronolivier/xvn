use crate::setup::shell_detection::Shell;
use anyhow::{Context, Result};
use std::env;
use std::io::IsTerminal;
use std::path::PathBuf;

/// Detect the user's current shell from environment
pub fn detect_shell() -> Result<Shell> {
    // Check $SHELL environment variable
    let shell_path = env::var("SHELL").context("SHELL environment variable not set")?;

    // Parse shell from path (e.g., /bin/zsh -> zsh)
    let shell_name = shell_path
        .rsplit('/')
        .next()
        .context("Invalid SHELL path")?;

    match shell_name {
        "bash" => Ok(Shell::Bash),
        "zsh" => Ok(Shell::Zsh),
        other => {
            log::warn!("Unknown shell: {other}, defaulting to bash");
            Ok(Shell::Bash)
        }
    }
}

/// Get the profile path for a given shell
pub fn get_profile_path(shell: &Shell) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;

    let profile_name = match shell {
        Shell::Bash => ".bashrc",
        Shell::Zsh => ".zshrc",
    };

    Ok(home.join(profile_name))
}

#[derive(Debug, Clone)]
pub struct DetectedManager {
    pub name: String,
    pub path: Option<PathBuf>,
}

/// Detect all installed version managers
pub fn detect_version_managers() -> Vec<DetectedManager> {
    let mut managers = Vec::new();

    if let Some(nvm) = check_nvm() {
        managers.push(nvm);
    }
    if let Some(fnm) = check_fnm() {
        managers.push(fnm);
    }
    if let Some(n) = check_n() {
        managers.push(n);
    }

    managers
}

/// Check if nvm is installed
fn check_nvm() -> Option<DetectedManager> {
    // Check $NVM_DIR first
    if let Ok(nvm_dir) = env::var("NVM_DIR") {
        let nvm_sh = PathBuf::from(&nvm_dir).join("nvm.sh");
        if nvm_sh.exists() {
            return Some(DetectedManager {
                name: "nvm".to_string(),
                path: Some(PathBuf::from(nvm_dir)),
            });
        }
    }

    // Check ~/.nvm
    if let Some(home) = dirs::home_dir() {
        let nvm_dir = home.join(".nvm");
        let nvm_sh = nvm_dir.join("nvm.sh");
        if nvm_sh.exists() {
            return Some(DetectedManager {
                name: "nvm".to_string(),
                path: Some(nvm_dir),
            });
        }
    }

    None
}

/// Check if fnm is installed
fn check_fnm() -> Option<DetectedManager> {
    use std::process::Command;

    // Try `which fnm`
    if let Ok(output) = Command::new("which").arg("fnm").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Some(DetectedManager {
                name: "fnm".to_string(),
                path: Some(PathBuf::from(path)),
            });
        }
    }

    // Check ~/.fnm
    if let Some(home) = dirs::home_dir() {
        let fnm_dir = home.join(".fnm");
        if fnm_dir.exists() {
            return Some(DetectedManager {
                name: "fnm".to_string(),
                path: Some(fnm_dir),
            });
        }
    }

    None
}

/// Check if n is installed
fn check_n() -> Option<DetectedManager> {
    use std::process::Command;

    // Try `which n`
    if let Ok(output) = Command::new("which").arg("n").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Some(DetectedManager {
                name: "n".to_string(),
                path: Some(PathBuf::from(path)),
            });
        }
    }

    None
}

/// Check if we're running in an interactive terminal
pub fn is_interactive() -> bool {
    std::io::stdin().is_terminal()
}

/// Check if we should run in interactive mode
/// Considers both TTY and --non-interactive flag
pub fn should_run_interactive(non_interactive_flag: bool) -> bool {
    !non_interactive_flag && is_interactive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_shell() {
        // This will use the actual environment
        // In CI, SHELL is typically /bin/bash
        let shell = detect_shell();
        assert!(shell.is_ok());
    }

    #[test]
    fn test_get_profile_path() {
        let bash_profile = get_profile_path(&Shell::Bash).unwrap();
        assert!(bash_profile.to_str().unwrap().ends_with(".bashrc"));

        let zsh_profile = get_profile_path(&Shell::Zsh).unwrap();
        assert!(zsh_profile.to_str().unwrap().ends_with(".zshrc"));
    }

    #[test]
    fn test_detect_managers() {
        // This will detect actual managers on the system
        let managers = detect_version_managers();

        // Should find at least one in dev environment
        // Don't assert on CI as it may not have any installed
        println!("Detected managers: {managers:?}");
    }

    #[test]
    fn test_should_run_interactive() {
        // When flag is true, should never be interactive
        assert!(!should_run_interactive(true));

        // When flag is false, depends on TTY
        // (Can't reliably test this in CI)
    }
}
