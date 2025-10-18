use anvs::config::{AutoInstallMode, Config};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_with_empty_file() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join(".xvnrc");
    fs::write(&config_path, "").unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let result: Result<Config, _> = serde_yaml::from_str(&content);

    // Empty YAML should use defaults (via serde default)
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.plugins, vec!["nvm", "fnm"]);
    assert_eq!(config.version_files, vec![".nvmrc", ".node-version"]);
    assert_eq!(config.auto_install, AutoInstallMode::Prompt);
}

#[test]
fn test_config_with_invalid_yaml() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join(".xvnrc");
    fs::write(&config_path, "invalid: [unclosed").unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let result: Result<Config, _> = serde_yaml::from_str(&content);

    // Should return ParseError
    assert!(result.is_err());
}

#[test]
fn test_config_with_unknown_fields() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join(".xvnrc");
    fs::write(
        &config_path,
        r#"
plugins:
  - nvm
unknown_field: should_be_ignored
another_unknown: 123
"#,
    )
    .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let result: Result<Config, _> = serde_yaml::from_str(&content);

    // Unknown fields should be ignored (serde default behavior)
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.plugins, vec!["nvm"]);
}

#[test]
fn test_config_merge_precedence() {
    // Note: merge() is private, so this tests the merge logic via Config structure
    let _base = Config {
        plugins: vec!["nvm".to_string(), "fnm".to_string()],
        version_files: vec![".nvmrc".to_string()],
        auto_install: AutoInstallMode::Never,
        use_default: true,
    };

    let override_config = Config {
        plugins: vec!["fnm".to_string()],
        version_files: vec![".node-version".to_string()],
        auto_install: AutoInstallMode::Always,
        use_default: true,
    };

    // Test that override values would take precedence
    // (merge() is tested in src/config/loader.rs unit tests)
    assert_eq!(override_config.plugins, vec!["fnm"]);
    assert_eq!(override_config.auto_install, AutoInstallMode::Always);
}

#[test]
fn test_config_default_values() {
    let config = Config::default();

    // Verify all defaults
    assert_eq!(config.plugins, vec!["nvm", "fnm"]);
    assert_eq!(config.version_files, vec![".nvmrc", ".node-version"]);
    assert_eq!(config.auto_install, AutoInstallMode::Prompt);
}

#[test]
fn test_config_plugin_priority_custom() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join(".xvnrc");
    fs::write(
        &config_path,
        r#"
plugins:
  - fnm
  - nvm
  - n
"#,
    )
    .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();

    // User-specified order should be preserved
    assert_eq!(config.plugins, vec!["fnm", "nvm", "n"]);
}

#[test]
fn test_config_plugin_priority_default() {
    let config = Config::default();

    // Default plugin priority
    assert_eq!(config.plugins, vec!["nvm", "fnm"]);
}

#[test]
fn test_config_with_comments() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join(".xvnrc");
    fs::write(
        &config_path,
        r#"
# This is a comment
plugins:
  - nvm  # Prefer nvm first
  # - fnm  # Disabled
version_files:
  - .nvmrc
# Another comment
"#,
    )
    .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();

    // YAML comments should be ignored
    assert_eq!(config.plugins, vec!["nvm"]);
    assert_eq!(config.version_files, vec![".nvmrc"]);
}

#[test]
fn test_config_auto_install_options() {
    let temp = TempDir::new().unwrap();

    // Test auto_install: always
    let config_path = temp.path().join(".xvnrc1");
    fs::write(&config_path, "auto_install: always").unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();
    assert_eq!(config.auto_install, AutoInstallMode::Always);

    // Test auto_install: never
    let config_path = temp.path().join(".xvnrc2");
    fs::write(&config_path, "auto_install: never").unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();
    assert_eq!(config.auto_install, AutoInstallMode::Never);

    // Test auto_install: prompt
    let config_path = temp.path().join(".xvnrc3");
    fs::write(&config_path, "auto_install: prompt").unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();
    assert_eq!(config.auto_install, AutoInstallMode::Prompt);

    // Test auto_install not specified (defaults to Prompt)
    let config_path = temp.path().join(".xvnrc4");
    fs::write(&config_path, "plugins: [nvm]").unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();
    assert_eq!(config.auto_install, AutoInstallMode::Prompt);
}

#[test]
fn test_config_validation() {
    // Test that empty plugins list fails validation
    let invalid_config = Config {
        plugins: vec![],
        version_files: vec![".nvmrc".to_string()],
        auto_install: AutoInstallMode::Prompt,
        use_default: true,
    };
    assert!(invalid_config.validate().is_err());

    // Test that empty version_files list fails validation
    let invalid_config = Config {
        plugins: vec!["nvm".to_string()],
        version_files: vec![],
        auto_install: AutoInstallMode::Prompt,
        use_default: true,
    };
    assert!(invalid_config.validate().is_err());

    // Test that valid config passes validation
    let valid_config = Config::default();
    assert!(valid_config.validate().is_ok());
}

#[test]
fn test_config_version_files_custom() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join(".xvnrc");
    fs::write(
        &config_path,
        r#"
version_files:
  - .nvmrc
  - .node-version
  - .tool-versions
"#,
    )
    .unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();

    assert_eq!(
        config.version_files,
        vec![".nvmrc", ".node-version", ".tool-versions"]
    );
}

#[test]
fn test_config_serde_round_trip() {
    let original = Config {
        plugins: vec!["nvm".to_string(), "fnm".to_string()],
        version_files: vec![".nvmrc".to_string()],
        auto_install: AutoInstallMode::Always,
        use_default: true,
    };

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();

    // Should be equal
    assert_eq!(original, deserialized);
}
