use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ai-switch"));
    assert!(stdout.contains("Manage AI provider configurations"));
}

#[test]
fn test_provider_list_empty() {
    // This test assumes a clean config, may need env var to override config path
    let output = Command::new("cargo")
        .args(["run", "--", "provider", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No providers"));
}
