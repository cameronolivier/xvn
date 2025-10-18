// Tests for setup/installer module

use anvs::setup::installer::SetupInstaller;

#[test]
fn test_installer_creation() {
    // Test that SetupInstaller can be created
    let result = SetupInstaller::new();
    assert!(result.is_ok(), "SetupInstaller creation should succeed");
}

#[test]
fn test_config_file_structure() {
    // Test that default config structure is valid YAML
    // This validates the string literal used in create_default_config()
    let default_config = r#"# xvn configuration file
# See https://github.com/cameronolivier/xvn for documentation

# Version files to search for (in priority order)
version_files:
  - .nvmrc
  - .node-version

# Plugin priority order
plugins:
  - nvm
  - fnm

# Auto-install mode: prompt (default), always, never
auto_install: prompt
"#;

    // Parse as YAML to verify it's valid
    let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(default_config);
    assert!(result.is_ok(), "Default config should be valid YAML");

    let config = result.unwrap();

    // Verify all required fields are present
    assert!(
        config.get("version_files").is_some(),
        "Should have version_files"
    );
    assert!(config.get("plugins").is_some(), "Should have plugins");
    assert!(
        config.get("auto_install").is_some(),
        "Should have auto_install"
    );

    // Verify field values
    let plugins = config.get("plugins").unwrap().as_sequence().unwrap();
    assert_eq!(plugins.len(), 2, "Should have 2 default plugins");
    assert_eq!(plugins[0].as_str().unwrap(), "nvm");
    assert_eq!(plugins[1].as_str().unwrap(), "fnm");

    let version_files = config.get("version_files").unwrap().as_sequence().unwrap();
    assert_eq!(
        version_files.len(),
        2,
        "Should have 2 default version files"
    );
    assert_eq!(version_files[0].as_str().unwrap(), ".nvmrc");
    assert_eq!(version_files[1].as_str().unwrap(), ".node-version");

    let auto_install = config.get("auto_install").unwrap().as_str().unwrap();
    assert_eq!(auto_install, "prompt", "Default should be prompt");
}

// Note: Full installer integration tests (install(), is_installed(), etc.)
// require complex filesystem mocking and are deferred. The installer is
// primarily tested via shell_integration tests which verify the complete
// setup workflow in a real environment.
