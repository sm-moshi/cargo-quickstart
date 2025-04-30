//! Integration test for CLI interface

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

// Helper function to create a temporary directory for testing
fn setup_test_dir(name: &str) -> String {
    let test_dir = format!("/tmp/cargo-quickstart-test-{}", name);
    let path = Path::new(&test_dir);
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
    fs::create_dir_all(path).unwrap();
    test_dir
}

#[test]
fn creates_project_successfully() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["new", "my-test-project", "--bin", "--yes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Generating project"));
}

#[test]
fn fails_with_missing_project_name() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn runs_help_successfully() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.arg("--help").assert().success();
}

// Additional tests using fixtures

#[test]
#[ignore = "Requires interactive input - for demonstration only"]
fn init_with_default_config() {
    // This test demonstrates how we would use the fixture for interactive input
    // It's ignored because it requires piping stdin which is complex in tests

    let test_dir = setup_test_dir("default-config");

    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    // In a real test, we would pipe the fixture file as stdin:
    // let input = include_str!("fixtures/default_config.txt");
    // cmd.write_stdin(input)

    cmd.args(["init", "--path", &test_dir]).assert().success();
}

#[test]
#[ignore = "Requires interactive input - for demonstration only"]
fn init_with_custom_config() {
    // This test demonstrates how we would use the fixture for interactive input
    // It's ignored because it requires piping stdin which is complex in tests

    let test_dir = setup_test_dir("custom-config");

    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    // In a real test, we would pipe the fixture file as stdin:
    // let input = include_str!("fixtures/custom_config.txt");
    // cmd.write_stdin(input)

    cmd.args(["init", "--path", &test_dir]).assert().success();
}

// Non-interactive tests (these will actually run)

#[test]
fn init_with_yes_flag() {
    let test_dir = setup_test_dir("yes-flag");

    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["init", "--path", &test_dir, "--yes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Generating project"));
}

#[test]
fn init_with_custom_values() {
    let test_dir = setup_test_dir("custom-values");

    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args([
        "init",
        "--path",
        &test_dir,
        "--name",
        "custom-name",
        "--lib",
        "--edition",
        "2018",
        "--license",
        "MIT",
        "--yes",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("custom-name"))
    .stdout(predicate::str::contains("Library"));
}
