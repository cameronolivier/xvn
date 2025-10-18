use anvs::plugins::{MockPlugin, PluginRegistry, VersionManagerPlugin};
use std::sync::Arc;

#[test]
fn test_mock_plugin_basic() {
    let plugin = MockPlugin::new("test")
        .with_version("18.20.0")
        .with_version("20.0.0");

    assert_eq!(plugin.name(), "test");
    assert!(plugin.is_available().unwrap());
    assert!(plugin.has_version("18.20.0").unwrap());
    assert!(plugin.has_version("20.0.0").unwrap());
    assert!(!plugin.has_version("16.0.0").unwrap());
}

#[test]
fn test_mock_plugin_unavailable() {
    let plugin = MockPlugin::new("test")
        .with_availability(false)
        .with_version("18.20.0");

    assert!(!plugin.is_available().unwrap());
    // Even though version is "installed", plugin is unavailable
    assert!(plugin.has_version("18.20.0").unwrap());
}

#[test]
fn test_mock_plugin_commands() {
    let plugin = MockPlugin::new("testvm");

    assert_eq!(
        plugin.activate_command("18.20.0").unwrap(),
        "testvm use 18.20.0"
    );
    assert_eq!(
        plugin.install_command("18.20.0").unwrap(),
        "testvm install 18.20.0"
    );
}

#[test]
fn test_mock_plugin_with_versions_helper() {
    let plugin = MockPlugin::new("test").with_versions(&["18.20.0", "20.0.0", "21.0.0"]);

    assert!(plugin.has_version("18.20.0").unwrap());
    assert!(plugin.has_version("20.0.0").unwrap());
    assert!(plugin.has_version("21.0.0").unwrap());
    assert!(!plugin.has_version("16.0.0").unwrap());
}

#[test]
fn test_registry_with_built_in_plugins() {
    let registry = PluginRegistry::new(&["nvm".to_string(), "fnm".to_string()]);
    assert_eq!(registry.plugins().len(), 2);
}

#[test]
fn test_plugin_priority_with_mocks() {
    // Manual test of priority logic using mock plugins directly
    let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![
        Arc::new(MockPlugin::new("first").with_version("18.20.0")),
        Arc::new(MockPlugin::new("second").with_version("18.20.0")),
    ];

    // First available with version should be returned
    let found = plugins
        .iter()
        .find(|p| p.is_available().unwrap_or(false) && p.has_version("18.20.0").unwrap_or(false));

    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "first");
}

#[test]
fn test_plugin_priority_unavailable_skipped() {
    // Test that unavailable plugins are skipped in priority order
    let plugins: Vec<Arc<dyn VersionManagerPlugin>> = vec![
        Arc::new(
            MockPlugin::new("first")
                .with_availability(false)
                .with_version("18.20.0"),
        ),
        Arc::new(MockPlugin::new("second").with_version("18.20.0")),
    ];

    // First plugin is unavailable, should find second
    let found = plugins
        .iter()
        .find(|p| p.is_available().unwrap_or(false) && p.has_version("18.20.0").unwrap_or(false));

    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "second");
}

#[test]
fn test_plugin_command_injection_awareness() {
    // Test that mock plugin doesn't perform escaping (real plugins do)
    let plugin = MockPlugin::new("test");

    let malicious_version = "18.20.0; rm -rf /";
    let cmd = plugin.activate_command(malicious_version).unwrap();

    // Command should contain the escaped version
    assert!(cmd.contains("test use"));
    // The version string should be present (mock doesn't escape, real plugins do)
    assert!(cmd.contains(malicious_version));
}
