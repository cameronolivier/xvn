mod fnm;
pub mod mock;
mod nvm;
mod registry;
mod trait_def;

pub use fnm::FnmPlugin;
pub use nvm::NvmPlugin;
pub use registry::PluginRegistry;
pub use trait_def::VersionManagerPlugin;

// Export MockPlugin for testing (both unit and integration tests)
#[doc(hidden)]
pub use mock::MockPlugin;
