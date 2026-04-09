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
fn test_provider_list() {
    let output = Command::new("cargo")
        .args(["run", "--", "provider", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Configured providers:") || stdout.contains("No providers configured"));
}

#[test]
fn test_status() {
    let output = Command::new("cargo")
        .args(["run", "--", "status"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AI Switch Status"));
}

#[test]
fn test_use_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "use", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--model"));
}
