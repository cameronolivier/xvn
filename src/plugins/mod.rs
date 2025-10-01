mod trait_def;
mod registry;
mod nvm;
mod fnm;
mod mock;

pub use trait_def::VersionManagerPlugin;
pub use registry::PluginRegistry;
pub use nvm::NvmPlugin;
pub use fnm::FnmPlugin;

// Export MockPlugin for testing (both unit and integration tests)
#[doc(hidden)]
pub use mock::MockPlugin;
