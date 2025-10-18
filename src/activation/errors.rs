use std::io;
use thiserror::Error;

/// Result type for activation operations
pub type ActivationResult<T> = Result<T, ActivationError>;

/// Errors that can occur during version activation
#[derive(Debug, Error)]
pub enum ActivationError {
    /// No version file found (not an error, just informational)
    #[error("no version file found")]
    NoVersionFile,

    /// Version file exists but is invalid
    #[error("invalid version file: {path}")]
    InvalidVersionFile {
        path: String,
        #[source]
        source: io::Error,
    },

    /// Version file is empty
    #[error("version file is empty: {path}")]
    EmptyVersionFile { path: String },

    /// Required version is not installed
    #[error("Node.js version {version} is not installed")]
    VersionNotInstalled { version: String, hint: String },

    /// No plugins are available on the system
    #[error("no version manager plugins available")]
    NoPluginsAvailable,

    /// Plugin-specific error
    #[error("plugin error ({plugin})")]
    PluginError {
        plugin: String,
        #[source]
        source: anyhow::Error,
    },

    /// Configuration error
    #[error("configuration error")]
    ConfigError(#[from] crate::error::AnvsError),

    /// I/O error
    #[error("I/O error")]
    IoError(#[from] io::Error),
}

impl ActivationError {
    /// Get an actionable hint for this error
    pub fn hint(&self) -> Option<String> {
        match self {
            Self::NoPluginsAvailable => Some(
                "Install a Node.js version manager:\n\
                 • nvm: https://github.com/nvm-sh/nvm\n\
                 • fnm: https://github.com/Schniz/fnm\n\
                 • n: https://github.com/tj/n"
                    .to_string(),
            ),
            Self::VersionNotInstalled { hint, .. } => Some(hint.clone()),
            Self::EmptyVersionFile { path } => Some(format!(
                "The version file '{path}' is empty.\n\
                 Add a Node.js version (e.g., '18.20.0') to the file."
            )),
            Self::InvalidVersionFile { path, .. } => Some(format!(
                "The version file '{path}' could not be read.\n\
                 Check file permissions and format."
            )),
            Self::ConfigError(_) => Some(
                "Run 'anvs setup' to create a default configuration, or check ~/.anvsrc syntax."
                    .to_string(),
            ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_hints_no_plugins() {
        let err = ActivationError::NoPluginsAvailable;
        let hint = err.hint().unwrap();
        assert!(hint.contains("nvm"));
        assert!(hint.contains("fnm"));
        assert!(hint.contains("https://"));
    }

    #[test]
    fn test_error_hints_version_not_installed() {
        let err = ActivationError::VersionNotInstalled {
            version: "18.20.0".to_string(),
            hint: "Run: nvm install 18.20.0".to_string(),
        };

        let msg = format!("{err}");
        assert!(msg.contains("18.20.0"));
        assert!(msg.contains("not installed"));

        let hint = err.hint().unwrap();
        assert_eq!(hint, "Run: nvm install 18.20.0");
    }

    #[test]
    fn test_error_hints_empty_version_file() {
        let err = ActivationError::EmptyVersionFile {
            path: "/tmp/test/.nvmrc".to_string(),
        };

        let msg = format!("{err}");
        assert!(msg.contains("/tmp/test/.nvmrc"));
        assert!(msg.contains("empty"));

        let hint = err.hint().unwrap();
        assert!(hint.contains("empty"));
        assert!(hint.contains("18.20.0")); // Example version
    }

    #[test]
    fn test_error_display() {
        let err = ActivationError::VersionNotInstalled {
            version: "18.20.0".to_string(),
            hint: "Install it".to_string(),
        };

        let msg = format!("{err}");
        assert!(msg.contains("18.20.0"));
        assert!(msg.contains("not installed"));
    }

    #[test]
    fn test_no_hint_for_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = ActivationError::IoError(io_err);
        assert!(err.hint().is_none());
    }
}
