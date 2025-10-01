// Integration tests for xvn
// These tests verify multi-component interactions

mod activation_scenarios {
    use std::fs;
    use std::sync::Arc;
    use tempfile::TempDir;
    use xvn::plugins::{MockPlugin, VersionManagerPlugin};
    use xvn::version_file::VersionFile;

    #[test]
    fn test_e2e_simple_activation() {
        // Test the full activation flow: find version file + find plugin with version
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join(".nvmrc"), "18.20.0\n").unwrap();

        let mock_plugin = MockPlugin::new("nvm")
            .with_version("18.20.0")
            .with_availability(true);

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![Arc::new(mock_plugin)];

        // Step 1: Find version file
        let version_file = VersionFile::find(temp.path(), &[".nvmrc".to_string()])
            .unwrap()
            .unwrap();

        assert_eq!(version_file.version, "18.20.0");

        // Step 2: Find plugin with version
        let found_plugin = plugins.iter().find(|p| {
            p.is_available().unwrap_or(false) && p.has_version("18.20.0").unwrap_or(false)
        });

        assert!(found_plugin.is_some());
        assert_eq!(found_plugin.unwrap().name(), "nvm");

        // Step 3: Generate activation command
        let activate_cmd = found_plugin.unwrap().activate_command("18.20.0").unwrap();
        assert_eq!(activate_cmd, "nvm use 18.20.0");
    }

    #[test]
    fn test_e2e_version_not_installed() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join(".nvmrc"), "99.99.99\n").unwrap();

        let mock_plugin = MockPlugin::new("nvm")
            .with_version("18.20.0")
            .with_availability(true);

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![Arc::new(mock_plugin)];

        let version_file = VersionFile::find(temp.path(), &[".nvmrc".to_string()])
            .unwrap()
            .unwrap();

        assert_eq!(version_file.version, "99.99.99");
        assert!(!plugins[0].has_version("99.99.99").unwrap_or(false));
    }

    #[test]
    fn test_e2e_nested_directory_search() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join(".nvmrc"), "20.0.0\n").unwrap();

        let nested = temp.path().join("src/components");
        fs::create_dir_all(&nested).unwrap();

        let version_file = VersionFile::find(&nested, &[".nvmrc".to_string()])
            .unwrap()
            .unwrap();

        assert_eq!(version_file.version, "20.0.0");
    }

    #[test]
    fn test_e2e_multiple_plugins_first_has_version() {
        let plugin1 = MockPlugin::new("nvm")
            .with_version("18.20.0")
            .with_availability(true);

        let plugin2 = MockPlugin::new("fnm")
            .with_version("20.0.0")
            .with_availability(true);

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> =
            vec![Arc::new(plugin1), Arc::new(plugin2)];

        let found_plugin = plugins
            .iter()
            .find(|p| p.has_version("18.20.0").unwrap_or(false));

        assert_eq!(found_plugin.unwrap().name(), "nvm");
    }
}

mod plugin_fallback {
    use std::sync::Arc;
    use xvn::plugins::{MockPlugin, VersionManagerPlugin};

    #[test]
    fn test_fallback_first_unavailable() {
        let nvm = MockPlugin::new("nvm")
            .with_availability(false)
            .with_version("18.20.0");

        let fnm = MockPlugin::new("fnm")
            .with_availability(true)
            .with_version("18.20.0");

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![Arc::new(nvm), Arc::new(fnm)];

        let found = plugins.iter().find(|p| {
            p.is_available().unwrap_or(false) && p.has_version("18.20.0").unwrap_or(false)
        });

        assert_eq!(found.unwrap().name(), "fnm");
    }

    #[test]
    fn test_fallback_first_no_version() {
        let nvm = MockPlugin::new("nvm")
            .with_availability(true)
            .with_version("20.0.0");

        let fnm = MockPlugin::new("fnm")
            .with_availability(true)
            .with_version("18.20.0");

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![Arc::new(nvm), Arc::new(fnm)];

        let found = plugins
            .iter()
            .find(|p| p.has_version("18.20.0").unwrap_or(false));

        assert_eq!(found.unwrap().name(), "fnm");
    }

    #[test]
    fn test_fallback_all_unavailable() {
        let nvm = MockPlugin::new("nvm")
            .with_availability(false)
            .with_version("18.20.0");

        let fnm = MockPlugin::new("fnm")
            .with_availability(false)
            .with_version("18.20.0");

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![Arc::new(nvm), Arc::new(fnm)];

        let found = plugins.iter().find(|p| p.is_available().unwrap_or(false));

        assert!(found.is_none());
    }

    #[test]
    fn test_fallback_respects_priority_order() {
        let first = MockPlugin::new("first")
            .with_availability(true)
            .with_version("18.20.0");

        let second = MockPlugin::new("second")
            .with_availability(true)
            .with_version("18.20.0");

        let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![Arc::new(first), Arc::new(second)];

        let found = plugins.iter().find(|p| {
            p.is_available().unwrap_or(false) && p.has_version("18.20.0").unwrap_or(false)
        });

        assert_eq!(found.unwrap().name(), "first");
    }
}

// Config file parsing tests
mod config_file_parsing {
    use std::fs;
    use tempfile::TempDir;
    use xvn::config::{AutoInstallMode, Config};

    #[test]
    fn test_config_file_validation() {
        // Test that invalid config files are rejected
        let temp = TempDir::new().unwrap();

        // Create invalid config (empty plugins)
        let config_path = temp.path().join("invalid.yaml");
        fs::write(
            &config_path,
            r#"
plugins: []
version_files: []
"#,
        )
        .unwrap();

        // load_from_file is private, so we test via serde + validation
        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = serde_yaml::from_str(&content).unwrap();
        let result = config.validate();

        assert!(result.is_err(), "Should reject invalid config");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("at least one"),
            "Error should mention validation requirement"
        );
    }

    #[test]
    fn test_config_file_parsing() {
        // Test that valid config files parse correctly
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("valid.yaml");
        fs::write(
            &config_path,
            r#"
plugins:
  - fnm
  - nvm
auto_install: always
version_files:
  - .node-version
  - .nvmrc
"#,
        )
        .unwrap();

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = serde_yaml::from_str(&content).unwrap();

        assert_eq!(config.plugins, vec!["fnm", "nvm"]);
        assert_eq!(config.auto_install, AutoInstallMode::Always);
        assert_eq!(config.version_files, vec![".node-version", ".nvmrc"]);
        assert!(config.validate().is_ok());
    }
}
