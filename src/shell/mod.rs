mod fd3;
mod json_writer;

pub use fd3::CommandWriter;
pub use json_writer::JsonCommandWriter;

#[cfg(test)]
pub use fd3::MockCommandWriter;

use serde::{Deserialize, Serialize};

/// Protocol for communicating activation commands to shell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputProtocol {
    /// Unix file descriptor #3 protocol (bash/zsh)
    Fd3,
    /// JSON protocol for Windows PowerShell
    Json,
}

impl OutputProtocol {
    /// Auto-detect protocol based on platform
    pub fn detect() -> Self {
        #[cfg(windows)]
        return OutputProtocol::Json;

        #[cfg(not(windows))]
        return OutputProtocol::Fd3;
    }

    /// Check if running in PowerShell (via environment)
    pub fn from_env() -> Self {
        if cfg!(windows) || std::env::var("PSModulePath").is_ok() {
            OutputProtocol::Json
        } else {
            OutputProtocol::Fd3
        }
    }
}

/// JSON structure for command output (used by PowerShell)
#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOutput {
    pub commands: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_detect() {
        let protocol = OutputProtocol::detect();
        #[cfg(windows)]
        assert_eq!(protocol, OutputProtocol::Json);
        #[cfg(not(windows))]
        assert_eq!(protocol, OutputProtocol::Fd3);
    }

    #[test]
    fn test_protocol_from_env() {
        // Without PowerShell env var
        std::env::remove_var("PSModulePath");
        let protocol = OutputProtocol::from_env();

        #[cfg(windows)]
        assert_eq!(protocol, OutputProtocol::Json);

        #[cfg(not(windows))]
        {
            assert_eq!(protocol, OutputProtocol::Fd3);

            // With PowerShell env var (simulating PowerShell on Unix)
            std::env::set_var("PSModulePath", "/usr/share/powershell");
            let protocol = OutputProtocol::from_env();
            assert_eq!(protocol, OutputProtocol::Json);
            std::env::remove_var("PSModulePath");
        }
    }
}
