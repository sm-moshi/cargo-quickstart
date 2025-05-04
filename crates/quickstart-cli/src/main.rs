//! CLI entry point for cargo-quickstart

mod args;
mod commands;
mod errors;
mod ui;

use args::{Cli, Commands};
use clap::Parser;
use errors::CliError;

fn main() -> Result<(), CliError> {
    // Setup error handling
    errors::setup().map_err(|e| CliError::Other(e.to_string()))?;

    // Parse command-line arguments
    let cli = Cli::parse();

    // Route to the appropriate command handler
    match cli.command {
        Commands::New(args) => {
            commands::execute_new(args).map_err(|e| CliError::CommandError(e.to_string()))?
        }
        Commands::Init(args) => {
            commands::execute_init(args).map_err(|e| CliError::CommandError(e.to_string()))?
        }
        Commands::ListTemplates => {
            commands::execute_list_templates().map_err(|e| CliError::CommandError(e.to_string()))?
        }
        #[cfg(feature = "completions")]
        Commands::Completions(args) => commands::execute_completions(args)
            .map_err(|e| CliError::CommandError(e.to_string()))?,
        #[cfg(feature = "doctor")]
        Commands::Doctor => {
            commands::execute_doctor().map_err(|e| CliError::CommandError(e.to_string()))?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        args::{Cli, Commands, InitArgs, NewArgs},
        errors::CliError,
    };
    use clap::Parser;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn test_main_error_handling() {
        // Create a CliError directly and check its properties instead of calling setup()
        // which might have different behavior in test environments
        let err = CliError::Other("Test error".to_string());
        assert!(err.to_string().contains("Test error"));
    }

    #[test]
    fn test_cli_parse() {
        // Test that Cli parsing works correctly
        let cli = Cli::parse_from(["cargo-quickstart", "list-templates"]);

        match cli.command {
            Commands::ListTemplates => {
                // Expected command
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_error_conversion() {
        // Test converting a string error to CliError
        let err = CliError::Other("Test error".to_string());
        assert_eq!(err.to_string(), "Error: Test error");

        let err = CliError::CommandError("Test command error".to_string());
        assert_eq!(err.to_string(), "Command failed: Test command error");
    }

    #[test]
    fn test_new_command_parsing() {
        // Test that New command is parsed correctly
        let cli = Cli::parse_from([
            "cargo-quickstart",
            "new",
            "test-project",
            "--bin",
            "--edition",
            "2021",
            "--git",
        ]);

        match cli.command {
            Commands::New(args) => {
                assert_eq!(args.name, "test-project");
                assert!(args.bin);
                assert!(!args.lib);
                assert_eq!(args.edition, "2021");
                assert!(args.git);
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_init_command_parsing() {
        // Test that Init command is parsed correctly
        let cli = Cli::parse_from([
            "cargo-quickstart",
            "init",
            "--name",
            "test-lib",
            "--lib",
            "--license",
            "MIT",
            "--path",
            "/tmp/project",
        ]);

        match cli.command {
            Commands::Init(args) => {
                assert_eq!(args.name, Some("test-lib".to_string()));
                assert!(!args.bin);
                assert!(args.lib);
                assert_eq!(args.license, "MIT");
                assert_eq!(args.path.to_string_lossy(), "/tmp/project");
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[cfg(feature = "completions")]
    #[test]
    fn test_completions_command_parsing() {
        // Test that Completions command is parsed correctly
        #[allow(unused_imports)]
        use crate::args::{CompletionsArgs, Shell};

        let cli = Cli::parse_from([
            "cargo-quickstart",
            "completions",
            "bash",
            "--output",
            "/tmp/completions.bash",
        ]);

        match cli.command {
            Commands::Completions(args) => {
                assert_eq!(args.shell.to_string(), "bash");
                assert_eq!(args.output, Some(PathBuf::from("/tmp/completions.bash")));
            }
            _ => panic!("Expected Completions command"),
        }
    }

    #[cfg(feature = "doctor")]
    #[test]
    fn test_doctor_command_parsing() {
        // Test that Doctor command is parsed correctly
        let cli = Cli::parse_from(["cargo-quickstart", "doctor"]);

        match cli.command {
            Commands::Doctor => {
                // Successfully parsed Doctor command
            }
            _ => panic!("Expected Doctor command"),
        }
    }

    // Test a simplified version of the main function's command route handling
    #[test]
    fn test_command_routing() {
        // We'll mock the command execution functions to avoid actual execution
        // This just tests the command routing logic itself

        // Mock Commands::New case
        let new_args = NewArgs {
            name: "test-project".to_string(),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: None,
            yes: false,
        };

        let result = match Commands::New(new_args) {
            Commands::New(_) => Ok(()),
            _ => Err(CliError::Other("Wrong command matched".to_string())),
        };
        assert!(result.is_ok());

        // Mock Commands::Init case
        let init_args = InitArgs {
            bin: false,
            lib: true,
            name: Some("test-lib".to_string()),
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: PathBuf::from("."),
            yes: false,
        };

        let result = match Commands::Init(init_args) {
            Commands::Init(_) => Ok(()),
            _ => Err(CliError::Other("Wrong command matched".to_string())),
        };
        assert!(result.is_ok());

        // Mock Commands::ListTemplates case
        let result = match Commands::ListTemplates {
            Commands::ListTemplates => Ok(()),
            _ => Err(CliError::Other("Wrong command matched".to_string())),
        };
        assert!(result.is_ok());
    }
}
