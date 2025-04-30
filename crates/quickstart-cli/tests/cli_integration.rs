//! Integration test for CLI interface

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

// Helper function to create a temporary directory for testing
fn setup_test_dir(name: &str) -> String {
    let test_dir = format!("target/test-tmp/{}", name);
    let path = Path::new(&test_dir);
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
    fs::create_dir_all(path).unwrap();
    test_dir
}

#[test]
fn creates_project_successfully() {
    let path = std::path::Path::new("target/test-tmp/my-test-project");
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["new", "target/test-tmp/my-test-project", "--bin", "--yes"])
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

#[test]
fn new_help_output() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["new", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Create a new Rust project"));
}

#[test]
fn init_help_output() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialise an existing directory"));
}

#[test]
fn unknown_subcommand_fails() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error").and(predicate::str::contains("unknown")));
}

#[test]
fn conflicting_flags_new() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["new", "proj", "--bin", "--lib"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn conflicting_flags_init() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["init", "--bin", "--lib"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn alias_new_works() {
    let path = std::path::Path::new("target/test-tmp/alias-proj");
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["n", "target/test-tmp/alias-proj", "--yes"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Generating project").or(predicate::str::contains("project")),
        );
}

#[test]
fn alias_init_works() {
    let test_dir = setup_test_dir("alias-init");
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["i", "--path", &test_dir, "--yes"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Generating project").or(predicate::str::contains("project")),
        );
}

#[test]
fn new_with_existing_path_fails() {
    let test_dir = setup_test_dir("existing-path");
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["new", test_dir.as_str(), "--yes"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists").or(predicate::str::contains("exists")));
}

#[test]
fn new_with_invalid_edition_fails() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args([
        "new",
        "bad-edition",
        "--edition",
        "not-a-real-edition",
        "--yes",
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains("edition").or(predicate::str::contains("invalid")));
}

#[test]
fn init_with_invalid_license_fails() {
    let test_dir = setup_test_dir("bad-license");
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args([
        "init",
        "--path",
        &test_dir,
        "--license",
        "not-a-real-license",
        "--yes",
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains("license").or(predicate::str::contains("invalid")));
}

#[test]
fn list_templates_works() {
    let mut cmd = Command::cargo_bin("cargo-quickstart").unwrap();
    cmd.args(["list-templates"]).assert().success().stdout(
        predicate::str::contains("Available templates")
            .or(predicate::str::contains("No templates found")),
    );
}
