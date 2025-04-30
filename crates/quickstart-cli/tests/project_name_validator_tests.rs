//! Tests for project name validation fixtures

// Test fixture files only
// Rather than directly calling code from the crate (which is difficult in integration tests),
// we're focusing on testing the fixtures themselves

use pretty_assertions::assert_eq;

/// Test reading from a fixture file
#[test]
fn test_fixture_exists() {
    let fixture = include_str!("fixtures/valid_project_name.txt");
    assert_eq!(fixture.trim(), "my-valid-project");
}

/// Test invalid project name fixtures
#[test]
fn test_invalid_fixtures_exist() {
    // Empty fixture
    let fixture = include_str!("fixtures/invalid_project_name_empty.txt");
    assert_eq!(fixture.trim(), "");

    // Non-alpha start
    let fixture = include_str!("fixtures/invalid_project_name_non_alpha_start.txt");
    assert_eq!(fixture.trim(), "1invalid-project");

    // Invalid characters
    let fixture = include_str!("fixtures/invalid_project_name_invalid_chars.txt");
    assert_eq!(fixture.trim(), "invalid@project");
}

// The full validation logic is tested in the init.rs tests
// These fixtures would be used in CLI integration tests that use std::process::Command
// to pipe fixture content as stdin to the application
