//! Implementation of the 'new' command for creating a new project

use color_eyre::Result;
use quickstart_lib::{generate_project, ProjectConfig, ProjectType};
use std::path::PathBuf;

use crate::{
    args::NewArgs,
    errors::CommandErrorExt,
    ui::{output, progress::with_spinner},
};

/// Execute the 'new' command
pub fn execute(args: NewArgs) -> Result<()> {
    output::header("Generating project");

    // Determine project type
    let project_type = if args.lib {
        ProjectType::Library
    } else {
        ProjectType::Binary // Default to binary if not specified
    };

    // Determine project path
    let project_path = if let Some(path) = args.path {
        path
    } else {
        PathBuf::from(&args.name)
    };

    // Check if the target directory already exists
    if project_path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Target directory '{}' already exists. Refusing to overwrite.",
            project_path.display()
        ));
    }

    // Display project information
    output::section("Project configuration");
    output::key_value("Name", &args.name);
    output::key_value("Type", &project_type.to_string());
    output::key_value("Edition", &args.edition);
    output::key_value("License", &args.license);
    output::key_value("Path", &project_path.display().to_string());
    output::key_value("Git", &args.git.to_string());

    // Build configuration
    let config = ProjectConfig {
        name: args.name,
        project_type,
        edition: args.edition,
        license: args.license,
        path: project_path,
        git: args.git,
        yes: args.yes,
    };

    // Generate project with a progress spinner
    with_spinner(
        "Generating project...",
        "Project created successfully!",
        || generate_project(config).command_context("new"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_environment() -> Result<(TempDir, std::path::PathBuf)> {
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
        std::env::set_current_dir(current_dir)?;
        Ok(())
    }

    #[test]
    fn test_execute_creates_project() -> Result<()> {
        let (temp_dir, current_dir) = setup_test_environment()?;
        let project_dir = temp_dir.path().join("test-project");

        let args = NewArgs {
            name: "test-project".to_string(),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: Some(project_dir.clone()),
            git: false,
            yes: true,
        };

        // Enable mocking for prompts
        crate::ui::prompts::enable_mocking();
        crate::ui::prompts::set_mock_confirm(Some(true));

        let result = execute(args);

        // Cleanup test environment
        cleanup_test_environment(current_dir)?;

        // Disable mocking
        crate::ui::prompts::disable_mocking();

        assert!(result.is_ok(), "execute() should succeed");
        assert!(project_dir.exists(), "Project directory should be created");
        assert!(
            project_dir.join("Cargo.toml").exists(),
            "Cargo.toml should be created"
        );

        Ok(())
    }

    #[test]
    fn test_execute_creates_library_project() -> Result<()> {
        let (temp_dir, current_dir) = setup_test_environment()?;
        let project_dir = temp_dir.path().join("test-lib");

        let args = NewArgs {
            name: "test-lib".to_string(),
            path: Some(project_dir.clone()),
            lib: true,
            bin: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            yes: true,
        };

        let result = execute(args);

        // Cleanup test environment
        cleanup_test_environment(current_dir)?;

        assert!(result.is_ok(), "execute() should succeed");
        assert!(project_dir.exists(), "Project directory should be created");
        assert!(
            project_dir.join("Cargo.toml").exists(),
            "Cargo.toml should be created"
        );
        assert!(
            project_dir.join("src").join("lib.rs").exists(),
            "lib.rs should be created"
        );

        Ok(())
    }

    #[test]
    fn test_execute_fails_on_existing_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project_path = temp_dir.path().join("existing-project");
        fs::create_dir(&project_path)?;

        let args = NewArgs {
            name: "existing-project".to_string(),
            path: Some(project_path),
            lib: false,
            bin: true,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            yes: false,
        };

        let result = execute(args);
        assert!(
            result.is_err(),
            "execute() should fail on existing directory"
        );

        Ok(())
    }
}
