use anvs::error::AnvsError;
use std::path::PathBuf;

#[test]
fn test_no_version_file_error() {
    let error = AnvsError::NoVersionFile {
        path: PathBuf::from("/tmp/test"),
    };

    let message = error.to_string();
    assert!(message.contains("no version file found"));
    assert!(message.contains("/tmp/test"));
}

#[test]
fn test_config_error() {
    let error = AnvsError::ConfigError {
        message: "invalid plugin name".to_string(),
    };

    let message = error.to_string();
    assert!(message.contains("configuration error"));
    assert!(message.contains("invalid plugin name"));
}

#[test]
fn test_plugin_error() {
    let error = AnvsError::PluginError {
        plugin: "nvm".to_string(),
        message: "not found".to_string(),
    };

    let message = error.to_string();
    assert!(message.contains("plugin 'nvm' failed"));
    assert!(message.contains("not found"));
}

#[test]
fn test_error_display_user_friendly() {
    // Test that error messages are user-friendly and actionable
    let errors = vec![
        AnvsError::NoVersionFile {
            path: PathBuf::from("/project"),
        },
        AnvsError::ConfigError {
            message: "no plugins configured".to_string(),
        },
        AnvsError::NoPluginAvailable {
            plugins: "nvm, fnm".to_string(),
        },
    ];

    for error in errors {
        let message = error.to_string();
        // Should not contain technical jargon or stack traces
        assert!(!message.contains("panic"));
        assert!(!message.contains("unwrap"));
        // Should be clear and informative
        assert!(!message.is_empty());
    }
}

#[test]
fn test_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let anvs_error: AnvsError = io_error.into();

    let message = anvs_error.to_string();
    assert!(message.contains("IO error"));
}

#[test]
fn test_error_from_yaml_parse_error() {
    // Create invalid YAML
    let invalid_yaml = "invalid: [unclosed";
    let yaml_error = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml).unwrap_err();
    let anvs_error: AnvsError = yaml_error.into();

    let message = anvs_error.to_string();
    assert!(message.contains("YAML parsing error"));
}

#[test]
fn test_error_no_plugin_available() {
    let error = AnvsError::NoPluginAvailable {
        plugins: "nvm, fnm, n".to_string(),
    };

    let message = error.to_string();
    assert!(message.contains("no version manager plugin available"));
    assert!(message.contains("nvm, fnm, n"));
}

#[test]
fn test_error_version_file_empty() {
    let error = AnvsError::VersionFileEmpty {
        path: PathBuf::from("/project/.nvmrc"),
    };

    let message = error.to_string();
    assert!(message.contains("version file is empty or invalid"));
    assert!(message.contains(".nvmrc"));
}

#[test]
fn test_error_version_file_unreadable() {
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
    let error = AnvsError::VersionFileUnreadable {
        path: PathBuf::from("/project/.nvmrc"),
        source: io_error,
    };

    let message = error.to_string();
    assert!(message.contains("failed to read version file"));
    assert!(message.contains(".nvmrc"));
}

#[test]
fn test_error_chain_preservation() {
    // Test that error chains are preserved (for debugging)
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = AnvsError::VersionFileUnreadable {
        path: PathBuf::from("/test/.nvmrc"),
        source: io_error,
    };

    // Error should have a source
    let source = std::error::Error::source(&error);
    assert!(source.is_some());
}

#[test]
fn test_error_debug_format() {
    let error = AnvsError::PluginError {
        plugin: "nvm".to_string(),
        message: "execution failed".to_string(),
    };

    let debug_str = format!("{error:?}");
    // Debug format should contain struct details
    assert!(debug_str.contains("PluginError"));
}

#[test]
fn test_error_result_type_alias() {
    // Test that Result<T> type alias works correctly
    fn returns_result() -> anvs::error::Result<String> {
        Ok("success".to_string())
    }

    let result = returns_result();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_error_result_type_alias_with_error() {
    fn returns_error() -> anvs::error::Result<String> {
        Err(AnvsError::ConfigError {
            message: "test error".to_string(),
        })
    }

    let result = returns_error();
    assert!(result.is_err());
}

#[test]
fn test_error_no_version_file_with_special_path() {
    let error = AnvsError::NoVersionFile {
        path: PathBuf::from("/path/with spaces/and-special_chars"),
    };

    let message = error.to_string();
    // Should handle paths with special characters
    assert!(message.contains("with spaces"));
}

#[test]
fn test_error_plugin_error_with_multiline_message() {
    let error = AnvsError::PluginError {
        plugin: "nvm".to_string(),
        message: "line 1\nline 2\nline 3".to_string(),
    };

    let message = error.to_string();
    // Should preserve multiline messages
    assert!(message.contains("line 1"));
    assert!(message.contains("line 2"));
}

#[test]
fn test_error_config_error_with_empty_message() {
    let error = AnvsError::ConfigError {
        message: String::new(),
    };

    let message = error.to_string();
    // Should still produce a message
    assert!(message.contains("configuration error"));
}

#[test]
fn test_error_implements_std_error_trait() {
    let error = AnvsError::PluginError {
        plugin: "nvm".to_string(),
        message: "test".to_string(),
    };

    // Should be able to use as std::error::Error
    let _error_trait: &dyn std::error::Error = &error;
}

#[test]
fn test_error_can_be_converted_with_anyhow() {
    let anvs_error = AnvsError::ConfigError {
        message: "test".to_string(),
    };

    // Should be able to convert to anyhow::Error
    let _anyhow_error: anyhow::Error = anvs_error.into();
}
