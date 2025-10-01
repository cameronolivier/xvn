use std::path::PathBuf;
use xvn::error::XvnError;

#[test]
fn test_no_version_file_error() {
    let error = XvnError::NoVersionFile {
        path: PathBuf::from("/tmp/test"),
    };

    let message = error.to_string();
    assert!(message.contains("no version file found"));
    assert!(message.contains("/tmp/test"));
}

#[test]
fn test_config_error() {
    let error = XvnError::ConfigError {
        message: "invalid plugin name".to_string(),
    };

    let message = error.to_string();
    assert!(message.contains("configuration error"));
    assert!(message.contains("invalid plugin name"));
}

#[test]
fn test_plugin_error() {
    let error = XvnError::PluginError {
        plugin: "nvm".to_string(),
        message: "not found".to_string(),
    };

    let message = error.to_string();
    assert!(message.contains("plugin 'nvm' failed"));
    assert!(message.contains("not found"));
}
