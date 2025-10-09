//! xvn - Extreme Version Switcher for Node.js
//!
//! Fast, modular automatic Node.js version switching.

pub mod activation;
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod init;
pub mod output;
pub mod plugins;
pub mod setup;
pub mod shell;
pub mod version_file;

// Re-export key types
pub use config::Config;
pub use error::XvnError;
pub use plugins::{PluginRegistry, VersionManagerPlugin};
pub use shell::CommandWriter;
pub use version_file::VersionFile;
