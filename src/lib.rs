//! ANVS - Automatic Node Version Switcher
//!
//! Fast, modular automatic Node.js version switching.
//!
//! ANVS automatically switches Node.js versions when you change directories,
//! reading from `.nvmrc`, `.node-version`, or `package.json` files.

pub mod activation;
pub mod cli;
pub mod commands;
pub mod config;
pub mod engines_resolver;
pub mod error;
pub mod init;
pub mod installation_detector;
pub mod output;
pub mod plugins;
pub mod setup;
pub mod shell;
pub mod version_file;

// Re-export key types
pub use config::Config;
pub use error::AnvsError;
pub use plugins::{PluginRegistry, VersionManagerPlugin};
pub use shell::CommandWriter;
pub use version_file::VersionFile;
