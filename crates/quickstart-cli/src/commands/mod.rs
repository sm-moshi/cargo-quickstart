//! Command implementations for cargo-quickstart

#[cfg(feature = "completions")]
mod completions;
#[cfg(feature = "doctor")]
mod doctor;
mod init;
mod new;
mod templates;

#[cfg(feature = "completions")]
use crate::args::CompletionsArgs;
use crate::args::{InitArgs, NewArgs};
use color_eyre::Result;

/// Execute the 'new' command
pub fn execute_new(args: NewArgs) -> Result<()> {
    new::execute(args)
}

/// Execute the 'init' command
pub fn execute_init(args: InitArgs) -> Result<()> {
    init::execute(args)
}

/// Execute the 'list-templates' command
pub fn execute_list_templates() -> Result<()> {
    // Find the templates directory
    let template_dir = quickstart_lib::find_templates_dir()?;
    let loader = quickstart_lib::template::TemplateLoader::new(template_dir);

    // Collect and display templates
    let all_templates = templates::collect_templates(&loader)?;
    templates::display_templates(all_templates);

    Ok(())
}

/// Execute the 'completions' command
#[cfg(feature = "completions")]
pub fn execute_completions(args: CompletionsArgs) -> color_eyre::Result<()> {
    completions::execute(args)
}

/// Execute the 'doctor' command
#[cfg(feature = "doctor")]
pub fn execute_doctor() -> color_eyre::Result<()> {
    doctor::execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{InitArgs, NewArgs};
    use color_eyre::{eyre::WrapErr, Result};
    use mockall::predicate::*;
    use mockall::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Create a mock for the quickstart_lib::template::TemplateLoader
    #[allow(clippy::disallowed_methods)]
    #[rustfmt::skip]
    mod mock_template_loader {
        use super::*;

        mock! {
            pub TemplateLoader {
                fn list_templates(&self) -> Result<Vec<String>>;
                fn new(template_dir: PathBuf) -> Self;
            }
        }
    }

    // Use the MockTemplateLoader in test_execute_list_templates_with_mocks
    #[allow(unused_imports)]
    use mock_template_loader::MockTemplateLoader;

    #[test]
    fn test_execute_list_templates_with_mocks() {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return;
        }

        // This is a simple test - we can't easily mock the entire quickstart_lib,
        // but we can verify the function doesn't panic when run
        // We're in a test environment, so there should be templates available
        let result = execute_list_templates();
        assert!(
            result.is_ok(),
            "execute_list_templates should complete successfully"
        );
    }

    #[test]
    #[cfg(feature = "doctor")]
    fn test_execute_doctor_returns_ok() {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return;
        }

        let result = execute_doctor();
        assert!(
            result.is_ok(),
            "execute_doctor should complete successfully"
        );
        // Explicitly drop any resources
        std::mem::drop(result);
    }

    #[cfg(feature = "completions")]
    #[test]
    fn test_execute_completions_returns_ok() {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return;
        }

        use crate::args::{CompletionsArgs, Shell};
        use std::io::{self, Write};

        // Test with stdout output
        let args = CompletionsArgs {
            shell: Shell::Bash,
            output: None,
        };

        let result = execute_completions(args);
        assert!(
            result.is_ok(),
            "execute_completions should complete successfully"
        );

        // Explicitly drop any resources to prevent memory leaks
        std::mem::drop(result);

        // Force stdout cleanup
        let _ = io::stdout().flush();
    }

    // Test that execute_new correctly passes its arguments to the new module
    // This is a unit test that verifies the function signature and call pattern
    #[test]
    fn test_execute_new_passes_args() {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return;
        }

        let args = NewArgs {
            name: "test-project".to_string(),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: Some(PathBuf::from("/non-existent-path/that-does-not-exist")),
            yes: true,
        };

        // We expect this to fail because we're using a non-existent path
        let result = execute_new(args);
        assert!(result.is_err(), "Should fail with non-existent path");
    }

    // Test that execute_init correctly passes its arguments to the init module
    #[test]
    fn test_execute_init_passes_args() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        // Create a temporary directory that we can write to
        let temp_dir = TempDir::new().wrap_err("Failed to create temporary directory")?;
        let nonexistent_subdir = temp_dir.path().join("nonexistent_subdir");

        let args = InitArgs {
            bin: true,
            lib: false,
            name: Some("test-project".to_string()),
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: nonexistent_subdir,
            yes: true,
            interactive: false,
        };

        // This test is only verifying that the function correctly passes arguments to the init module.
        // The implementation creates directories that don't exist, so this should succeed.
        let result = execute_init(args);
        assert!(
            result.is_ok(),
            "Should succeed because implementation creates directories"
        );

        Ok(())
    }
}
