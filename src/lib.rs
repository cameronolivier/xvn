//! xvn - Extreme Version Switcher for Node.js
//!
//! Fast, modular automatic Node.js version switching.

pub mod cli;
pub mod config;
pub mod error;
pub mod plugins;
pub mod version_file;

// Re-export key types
pub use config::Config;
pub use error::XvnError;
pub use plugins::VersionManagerPlugin;
pub use version_file::VersionFile;
