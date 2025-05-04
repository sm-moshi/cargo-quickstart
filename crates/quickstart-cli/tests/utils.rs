#![allow(clippy::disallowed_methods)]
//! Test utilities for CLI integration tests

use assert_cmd::Command;
use std::path::Path;

#[allow(clippy::disallowed_methods)]
/// Create a new command instance for testing
pub fn create_test_command() -> Command {
    Command::cargo_bin("cargo-quickstart").unwrap()
}

/// Helper to create a temporary project for testing
pub fn create_temp_project() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

/// Helper to assert command success with expected output
pub fn assert_success_with_output(cmd: &mut Command, expected: &str) {
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(expected));
}

#[allow(dead_code)]
/// Helper to assert command failure with expected error
pub fn assert_failure_with_error(cmd: &mut Command, expected: &str) {
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains(expected));
}

#[allow(dead_code)]
/// Helper to check if a file exists in a directory
pub fn assert_file_exists(dir: &Path, file: &str) {
    assert!(dir.join(file).exists(), "Expected file {file} to exist");
}
