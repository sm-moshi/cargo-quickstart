//! Tests for the init command

#[cfg(test)]
mod test {
    use crate::commands::init::{config, executor::execute};
    use crate::{args::InitArgs, ui::prompts};
    use color_eyre::eyre::Context;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;
    use quickstart_lib::ProjectType;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup_test_environment() -> Result<(TempDir, std::path::PathBuf)> {
        // Skip under Miri
        if cfg!(miri) {
            return Err(color_eyre::eyre::eyre!(
                "Skipping file system tests under Miri"
            ));
        }

        // Create a temporary directory for the test
        let temp_dir = TempDir::new()?;

        // Save current directory
        let current_dir = std::env::current_dir()?;

        // Create a test workspace in the temporary directory
        let test_workspace = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&test_workspace)?;

        // Create templates directory in the test workspace
        let templates_dir = test_workspace.join("templates");
        std::fs::create_dir_all(&templates_dir)?;

        // Create template structure for both minimal and extended variants
        std::fs::create_dir_all(templates_dir.join("base"))?;
        std::fs::create_dir_all(templates_dir.join("binary/minimal/src"))?;
        std::fs::create_dir_all(templates_dir.join("binary/extended/src"))?;
        std::fs::create_dir_all(templates_dir.join("library/minimal/src"))?;
        std::fs::create_dir_all(templates_dir.join("library/extended/src"))?;

        // Create basic template files
        std::fs::write(
            templates_dir.join("base/Cargo.toml.hbs"),
            r#"[package]
name = "{{name}}"
version = "0.1.0"
edition = "{{edition}}"
"#,
        )?;

        // Create minimal binary template
        std::fs::write(
            templates_dir.join("binary/minimal/src/main.rs.hbs"),
            "fn main() {\n    println!(\"Hello from {{name}}!\");\n}\n",
        )?;

        // Create extended binary template
        std::fs::write(
            templates_dir.join("binary/extended/src/main.rs.hbs"),
            r#"use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Name to greet
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello from {{name}}, {}!", args.name);
}
"#,
        )?;

        // Create minimal library template
        std::fs::write(
            templates_dir.join("library/minimal/src/lib.rs.hbs"),
            "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n",
        )?;

        // Create extended library template
        std::fs::write(
            templates_dir.join("library/extended/src/lib.rs.hbs"),
            r#"//! {{name}} library
//!
//! This library provides a set of utilities for working with numbers.

/// Adds two numbers together
///
/// # Examples
///
/// ```
/// let result = {{name}}::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
"#,
        )?;

        // Change to test workspace
        std::env::set_current_dir(&test_workspace)?;

        Ok((temp_dir, current_dir))
    }

    fn cleanup_test_environment(current_dir: std::path::PathBuf) -> Result<()> {
        if cfg!(miri) {
            return Ok(());
        }
        std::env::set_current_dir(current_dir)?;
        Ok(())
    }

    #[test]
    fn test_execute_with_confirmation() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let (temp_dir, current_dir) = setup_test_environment()?;

        let args = InitArgs {
            name: Some("test-project".to_string()),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: temp_dir.path().to_path_buf(),
            git: false,
            yes: false,
            interactive: false,
        };

        // Enable mocking for prompts
        prompts::enable_mocking();
        prompts::set_mock_confirm(Some(true));

        let result = execute(args);

        // Cleanup test environment
        cleanup_test_environment(current_dir)?;

        // Disable mocking
        prompts::disable_mocking();

        assert!(result.is_ok(), "execute() should succeed");
        Ok(())
    }

    #[test]
    fn test_execute_user_cancellation() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let (temp_dir, current_dir) = setup_test_environment()?;

        let args = InitArgs {
            name: Some("test-project".to_string()),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: temp_dir.path().to_path_buf(),
            git: false,
            yes: false,
            interactive: false,
        };

        // Enable mocking and set confirmation to false
        prompts::enable_mocking();
        prompts::set_mock_confirm(Some(false));

        let result = execute(args);

        // Cleanup test environment
        cleanup_test_environment(current_dir)?;

        // Disable mocking
        prompts::disable_mocking();

        assert!(result.is_err(), "execute() should fail when user cancels");

        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "Project initialization cancelled by user",
                "Unexpected error message"
            );
        }

        Ok(())
    }

    #[test]
    fn test_execute_nonexistent_directory() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let (temp_dir, current_dir) = setup_test_environment()?;
        let nonexistent_dir = temp_dir.path().join("nonexistent");

        let args = InitArgs {
            name: Some("test-project".to_string()),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: nonexistent_dir.clone(),
            git: false,
            yes: true,
            interactive: false,
        };

        let result = execute(args);

        // Cleanup test environment
        cleanup_test_environment(current_dir)?;

        assert!(
            result.is_ok(),
            "execute() should succeed with nonexistent directory"
        );
        assert!(nonexistent_dir.exists(), "Directory should be created");

        Ok(())
    }

    // Keep existing unit tests for config functions
    #[test]
    fn test_explicit_project_name() -> color_eyre::Result<()> {
        let args = InitArgs {
            name: Some("explicit-name".to_string()),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
            interactive: false,
        };

        let project_name =
            config::get_project_name(&args).wrap_err("Failed to get project name")?;
        assert_eq!(project_name, "explicit-name");
        Ok(())
    }

    #[test]
    fn test_prompted_name() -> color_eyre::Result<()> {
        let args = InitArgs {
            name: None,
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
            interactive: false,
        };

        prompts::enable_mocking();
        prompts::set_mock_input(Some("prompted-name".to_string()));

        let project_name =
            config::get_project_name(&args).wrap_err("Failed to get prompted project name")?;
        assert_eq!(project_name, "prompted-name");

        prompts::disable_mocking();
        Ok(())
    }

    #[test]
    fn test_lib_project_type() -> color_eyre::Result<()> {
        let args = InitArgs {
            name: None,
            bin: false,
            lib: true,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
            interactive: false,
        };

        let project_type =
            config::determine_project_type(&args).wrap_err("Failed to determine project type")?;
        assert_eq!(project_type, ProjectType::Library);
        Ok(())
    }

    #[test]
    fn test_bin_project_type() -> color_eyre::Result<()> {
        let args = InitArgs {
            name: None,
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
            interactive: false,
        };

        let project_type =
            config::determine_project_type(&args).wrap_err("Failed to determine project type")?;
        assert_eq!(project_type, ProjectType::Binary);
        Ok(())
    }

    #[test]
    fn test_prompted_project_type() -> color_eyre::Result<()> {
        let args = InitArgs {
            name: None,
            bin: false,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
            interactive: false,
        };

        prompts::enable_mocking();
        prompts::set_mock_select(Some(1)); // Select Library

        let project_type = config::determine_project_type(&args)
            .wrap_err("Failed to determine prompted project type")?;
        assert_eq!(project_type, ProjectType::Library);

        prompts::disable_mocking();
        Ok(())
    }

    #[test]
    fn test_fallback_project_type() -> color_eyre::Result<()> {
        let args = InitArgs {
            name: None,
            bin: false,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
            interactive: false,
        };

        prompts::enable_mocking();
        prompts::set_mock_select(Some(99)); // Invalid selection

        let project_type = config::determine_project_type(&args)
            .wrap_err("Failed to determine fallback project type")?;
        assert_eq!(project_type, ProjectType::Binary); // Should fallback to Binary

        prompts::disable_mocking();
        Ok(())
    }
}
