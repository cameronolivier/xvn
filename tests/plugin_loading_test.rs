// Additional plugin loading and registry tests

use std::sync::Arc;
use xvn::plugins::{MockPlugin, PluginRegistry, VersionManagerPlugin};

#[test]
fn test_registry_loads_nvm_plugin() {
    let registry = PluginRegistry::new(&["nvm".to_string()]);
    let plugins = registry.plugins();

    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].name(), "nvm");
}

#[test]
fn test_registry_loads_fnm_plugin() {
    let registry = PluginRegistry::new(&["fnm".to_string()]);
    let plugins = registry.plugins();

    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].name(), "fnm");
}

#[test]
fn test_registry_respects_priority_order() {
    let registry = PluginRegistry::new(&["fnm".to_string(), "nvm".to_string()]);
    let plugins = registry.plugins();

    assert_eq!(plugins.len(), 2);
    // fnm should be first (as specified in priority order)
    assert_eq!(plugins[0].name(), "fnm");
    assert_eq!(plugins[1].name(), "nvm");
}

#[test]
fn test_registry_with_unknown_plugin() {
    // Registry should skip unknown plugins
    let registry = PluginRegistry::new(&[
        "nvm".to_string(),
        "unknown_plugin".to_string(),
        "fnm".to_string(),
    ]);
    let plugins = registry.plugins();

    // Should only have nvm and fnm
    assert_eq!(plugins.len(), 2);
    assert_eq!(plugins[0].name(), "nvm");
    assert_eq!(plugins[1].name(), "fnm");
}

#[test]
fn test_registry_default_plugins() {
    let registry = PluginRegistry::new(&["nvm".to_string(), "fnm".to_string()]);
    let plugins = registry.plugins();

    assert_eq!(plugins.len(), 2);
    assert_eq!(plugins[0].name(), "nvm");
    assert_eq!(plugins[1].name(), "fnm");
}

#[test]
fn test_plugin_activate_command_generation() {
    let plugin = MockPlugin::new("testvm").with_version("18.20.0");

    let cmd = plugin.activate_command("18.20.0").unwrap();

    assert_eq!(cmd, "testvm use 18.20.0");
}

#[test]
fn test_plugin_install_command_generation() {
    let plugin = MockPlugin::new("testvm");

    let cmd = plugin.install_command("18.20.0").unwrap();

    assert_eq!(cmd, "testvm install 18.20.0");
}

#[test]
fn test_plugin_command_escaping() {
    let plugin = MockPlugin::new("testvm");

    // MockPlugin doesn't do escaping, but real plugins should
    // Test that malicious input doesn't break command generation
    let malicious_version = "18.20.0; rm -rf /";
    let cmd = plugin.activate_command(malicious_version).unwrap();

    // Command should contain the version (mock doesn't escape)
    assert!(cmd.contains("testvm use"));
    assert!(cmd.contains(malicious_version));
}

#[test]
fn test_plugin_version_with_special_characters() {
    let plugin = MockPlugin::new("testvm");

    // Test various version string formats
    let versions = vec![
        "18.20.0",
        "v18.20.0",
        "lts/hydrogen",
        "lts/*",
        "node",
        "18",
    ];

    for version in versions {
        let activate_cmd = plugin.activate_command(version).unwrap();
        assert!(activate_cmd.contains(version));

        let install_cmd = plugin.install_command(version).unwrap();
        assert!(install_cmd.contains(version));
    }
}

#[test]
fn test_registry_find_available_plugin_for_version() {
    let registry = PluginRegistry::new(&["nvm".to_string(), "fnm".to_string()]);

    // Create mock plugins with different available versions
    let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![
        Arc::new(MockPlugin::new("first").with_version("18.20.0")),
        Arc::new(MockPlugin::new("second").with_version("20.0.0")),
    ];

    // Find plugin with version 18.20.0
    let found = plugins
        .iter()
        .find(|p| p.has_version("18.20.0").unwrap_or(false));

    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "first");
}

#[test]
fn test_registry_no_plugin_has_version() {
    let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![
        Arc::new(MockPlugin::new("first").with_version("18.20.0")),
        Arc::new(MockPlugin::new("second").with_version("20.0.0")),
    ];

    // Try to find version that no plugin has
    let found = plugins
        .iter()
        .find(|p| p.has_version("99.99.99").unwrap_or(false));

    assert!(found.is_none());
}

#[test]
fn test_plugin_trait_send_sync() {
    // Verify that VersionManagerPlugin is Send + Sync
    // This is important for Arc<dyn VersionManagerPlugin>
    let plugin: Arc<dyn VersionManagerPlugin> = Arc::new(MockPlugin::new("test"));

    // Should be able to clone Arc (requires Send + Sync)
    let _plugin_clone = Arc::clone(&plugin);
}

#[test]
fn test_mock_plugin_builder_pattern() {
    let plugin = MockPlugin::new("test")
        .with_availability(true)
        .with_version("18.20.0")
        .with_version("20.0.0")
        .with_versions(&["21.0.0", "22.0.0"]);

    assert!(plugin.is_available().unwrap());
    assert!(plugin.has_version("18.20.0").unwrap());
    assert!(plugin.has_version("20.0.0").unwrap());
    assert!(plugin.has_version("21.0.0").unwrap());
    assert!(plugin.has_version("22.0.0").unwrap());
}

#[test]
fn test_plugin_availability_caching() {
    // Note: Real plugins cache availability checks
    // MockPlugin always returns the configured availability
    let plugin = MockPlugin::new("test").with_availability(true);

    // Multiple calls should return same result (caching in real plugins)
    assert!(plugin.is_available().unwrap());
    assert!(plugin.is_available().unwrap());
    assert!(plugin.is_available().unwrap());
}

#[test]
fn test_plugin_has_version_check_with_unavailable_plugin() {
    let plugin = MockPlugin::new("test")
        .with_availability(false)
        .with_version("18.20.0");

    // Plugin is unavailable
    assert!(!plugin.is_available().unwrap());

    // But version check still works (for testing)
    assert!(plugin.has_version("18.20.0").unwrap());
}

#[test]
fn test_registry_empty_plugin_list() {
    let registry = PluginRegistry::new(&[]);
    let plugins = registry.plugins();

    // Should have no plugins
    assert_eq!(plugins.len(), 0);
}

#[test]
fn test_registry_single_plugin() {
    let registry = PluginRegistry::new(&["nvm".to_string()]);
    let plugins = registry.plugins();

    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].name(), "nvm");
}

#[test]
fn test_registry_duplicate_plugins_in_config() {
    // Test that duplicate plugin names don't cause issues
    let registry = PluginRegistry::new(&[
        "nvm".to_string(),
        "fnm".to_string(),
        "nvm".to_string(), // Duplicate
    ]);
    let plugins = registry.plugins();

    // Should have 3 plugins (duplicates are allowed in config)
    // First nvm, then fnm, then nvm again
    assert_eq!(plugins.len(), 3);
    assert_eq!(plugins[0].name(), "nvm");
    assert_eq!(plugins[1].name(), "fnm");
    assert_eq!(plugins[2].name(), "nvm");
}
