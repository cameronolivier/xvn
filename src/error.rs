use std::path::PathBuf;
use thiserror::Error;

/// Main error type for anvs
#[derive(Debug, Error)]
pub enum XvnError {
    /// No version file found in directory tree
    #[error("no version file found in {path} or parent directories")]
    NoVersionFile { path: PathBuf },

    /// Version file exists but is empty or invalid
    #[error("version file is empty or invalid: {path}")]
    VersionFileEmpty { path: PathBuf },

    /// Failed to read version file
    #[error("failed to read version file: {path}")]
    VersionFileUnreadable {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Configuration file is invalid
    #[error("configuration error: {message}")]
    ConfigError { message: String },

    /// No version manager plugin is available
    #[error("no version manager plugin available (tried: {plugins})")]
    NoPluginAvailable { plugins: String },

    /// Plugin execution failed
    #[error("plugin '{plugin}' failed: {message}")]
    PluginError { plugin: String, message: String },

    /// IO error (generic wrapper)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

/// Result type alias for anvs operations
pub type Result<T> = std::result::Result<T, XvnError>;

/// Temporary alias while transitioning from the old XvnError name.
pub type AnvsError = XvnError;
