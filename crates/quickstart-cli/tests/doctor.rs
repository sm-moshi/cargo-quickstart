//! Integration tests for the doctor command

mod utils;
use anyhow::Result;
use utils::*;

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_runs() -> Result<()> {
    // Skip under Miri
    if cfg!(miri) {
        eprintln!("Skipping integration test under Miri");
        return Ok(());
    }

    let mut cmd = create_test_command()?;
    cmd.arg("doctor");
    assert_success_with_output(&mut cmd, "Project Health Summary");
    drop(cmd);
    Ok(())
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_shows_health_score() -> Result<()> {
    // Skip under Miri
    if cfg!(miri) {
        eprintln!("Skipping integration test under Miri");
        return Ok(());
    }

    let mut cmd = create_test_command()?;
    cmd.arg("doctor");
    assert_success_with_output(&mut cmd, "Health score:");
    drop(cmd);
    Ok(())
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_shows_statistics() -> Result<()> {
    // Skip under Miri
    if cfg!(miri) {
        eprintln!("Skipping integration test under Miri");
        return Ok(());
    }

    let mut cmd = create_test_command()?;
    cmd.arg("doctor");
    assert_success_with_output(&mut cmd, "Total checks:");
    drop(cmd);
    Ok(())
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_in_empty_directory() -> Result<()> {
    // Skip under Miri
    if cfg!(miri) {
        eprintln!("Skipping integration test under Miri");
        return Ok(());
    }

    let temp = create_temp_project()?;
    let mut cmd = create_test_command()?;
    cmd.arg("doctor").current_dir(temp.path());
    assert_success_with_output(&mut cmd, "Project Health Summary");
    drop(cmd);
    Ok(())
}

#[test]
#[cfg(feature = "doctor")]
fn test_doctor_command_shows_all_check_categories() -> Result<()> {
    // Skip under Miri
    if cfg!(miri) {
        eprintln!("Skipping integration test under Miri");
        return Ok(());
    }

    let mut cmd = create_test_command()?;
    cmd.arg("doctor");
    let assert = cmd.assert().success();
    assert
        .stdout(predicates::str::contains("Required Files"))
        .stdout(predicates::str::contains("Dependencies"))
        .stdout(predicates::str::contains("Rust Toolchain"))
        .stdout(predicates::str::contains("Templates"))
        .stdout(predicates::str::contains("Linting"));
    Ok(())
}
