//! Integration tests for the doctor command

mod utils;
use utils::*;

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_runs() {
    let mut cmd = create_test_command();
    cmd.arg("doctor");
    assert_success_with_output(&mut cmd, "Project Health Summary");
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_shows_health_score() {
    let mut cmd = create_test_command();
    cmd.arg("doctor");
    assert_success_with_output(&mut cmd, "Health score:");
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_shows_statistics() {
    let mut cmd = create_test_command();
    cmd.arg("doctor");
    assert_success_with_output(&mut cmd, "Total checks:");
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_in_empty_directory() {
    let temp = create_temp_project();
    let mut cmd = create_test_command();
    cmd.arg("doctor").current_dir(temp.path());
    assert_success_with_output(&mut cmd, "Project Health Summary");
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_shows_all_check_categories() {
    let mut cmd = create_test_command();
    cmd.arg("doctor");
    let assert = cmd.assert().success();
    assert
        .stdout(predicates::str::contains("Required Files"))
        .stdout(predicates::str::contains("Dependencies"))
        .stdout(predicates::str::contains("Rust Toolchain"))
        .stdout(predicates::str::contains("Templates"))
        .stdout(predicates::str::contains("Linting"));
}
