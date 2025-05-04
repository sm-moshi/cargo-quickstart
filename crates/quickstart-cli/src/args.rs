//! Command-line argument definitions for cargo-quickstart

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// A cargo subcommand for quickly generating opinionated Rust projects
#[derive(Parser, Debug)]
#[command(
    name = "cargo-quickstart",
    bin_name = "cargo-quickstart",
    version,
    about = "Opinionated cargo subcommand for bootstrapping modern Rust projects",
    long_about = "A blazing fast and opinionated cargo subcommand for bootstrapping modern Rust projects with confidence and speed.\n\nEXAMPLES:\n  cargo quickstart new my-app --bin --git\n  cargo quickstart init --lib --name my-lib --git\n\nSee https://github.com/smeya/cargo-quickstart for more info."
)]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands supported by cargo-quickstart
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Rust project with best-practice defaults
    #[command(
        name = "new",
        visible_alias = "n",
        about = "Create a new Rust project with best-practice defaults"
    )]
    New(NewArgs),

    /// Initialise an existing directory with a Rust project
    #[command(
        name = "init",
        visible_alias = "i",
        about = "Initialise an existing directory with a Rust project"
    )]
    Init(InitArgs),

    /// List all available project templates
    #[command(
        name = "list-templates",
        visible_alias = "ls",
        about = "List all available project templates"
    )]
    ListTemplates,

    /// Generate shell completion scripts for your shell
    #[cfg(feature = "completions")]
    #[command(
        name = "completions",
        about = "Generate shell completion scripts for your shell (bash, zsh, fish, powershell, elvish)",
        long_about = "Generate shell completion scripts for your shell. Example: cargo quickstart completions bash > /usr/local/etc/bash_completion.d/cargo-quickstart"
    )]
    Completions(CompletionsArgs),

    /// Diagnose common project issues and misconfigurations
    #[cfg(feature = "doctor")]
    #[command(
        name = "doctor",
        about = "Diagnose common project issues and misconfigurations",
        long_about = "Run a series of checks to validate your Rust project, templates, and environment. Reports missing files, misconfigurations, and actionable suggestions."
    )]
    Doctor,
}

/// Arguments for the 'new' command
#[derive(Args, Debug)]
pub struct NewArgs {
    /// Name of the new project (directory will be created)
    #[arg(help = "Name of the new project (directory will be created)")]
    pub name: String,

    /// Create a binary application (default if neither --bin nor --lib is specified)
    #[arg(
        long,
        help = "Create a binary (application) project",
        conflicts_with = "lib"
    )]
    pub bin: bool,

    /// Create a library crate
    #[arg(
        long,
        help = "Create a library (crate) project",
        conflicts_with = "bin"
    )]
    pub lib: bool,

    /// Rust edition to use (default: 2021)
    #[arg(long, help = "Rust edition (default: 2021)", default_value = "2021", value_parser = validate_edition)]
    pub edition: String,

    /// License to apply (MIT, Apache-2.0, or dual MIT/Apache-2.0)
    #[arg(long, help = "License to use (default: MIT OR Apache-2.0)", default_value = "MIT OR Apache-2.0", value_parser = validate_license)]
    pub license: String,

    /// Initialise a Git repository
    #[arg(long, help = "Initialise a Git repository")]
    pub git: bool,

    /// Target directory (defaults to the project name in the current directory)
    #[arg(
        long,
        help = "Target directory (defaults to project name in current directory)"
    )]
    pub path: Option<PathBuf>,

    /// Accept all defaults without prompting
    #[arg(short, long, help = "Accept all defaults without prompting")]
    pub yes: bool,
}

/// Arguments for the 'init' command
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Create a binary application (default if neither --bin nor --lib is specified)
    #[arg(
        long,
        help = "Create a binary (application) project",
        conflicts_with = "lib"
    )]
    pub bin: bool,

    /// Create a library crate
    #[arg(
        long,
        help = "Create a library (crate) project",
        conflicts_with = "bin"
    )]
    pub lib: bool,

    /// Name of the project (defaults to directory name)
    #[arg(long, help = "Project name (defaults to directory name)")]
    pub name: Option<String>,

    /// Rust edition to use (default: 2021)
    #[arg(long, help = "Rust edition (default: 2021)", default_value = "2021", value_parser = validate_edition)]
    pub edition: String,

    /// License to apply (MIT, Apache-2.0, or dual MIT/Apache-2.0)
    #[arg(long, help = "License to use (default: MIT OR Apache-2.0)", default_value = "MIT OR Apache-2.0", value_parser = validate_license)]
    pub license: String,

    /// Initialise a Git repository
    #[arg(long, help = "Initialise a Git repository")]
    pub git: bool,

    /// Target directory (defaults to the current directory)
    #[arg(
        long,
        help = "Target directory (defaults to current directory)",
        default_value = "."
    )]
    pub path: PathBuf,

    /// Accept all defaults without prompting
    #[arg(short, long, help = "Accept all defaults without prompting")]
    pub yes: bool,
}

/// Arguments for the 'completions' command
#[cfg(feature = "completions")]
#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// The shell to generate completions for (bash, zsh, fish, powershell, elvish)
    #[arg(value_enum, help = "Shell type (bash, zsh, fish, powershell, elvish)")]
    pub shell: Shell,

    /// Output file path (optional, defaults to stdout)
    #[arg(long, help = "Output file path (optional, defaults to stdout)")]
    pub output: Option<PathBuf>,
}

/// Supported shells for completions
#[cfg(feature = "completions")]
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Shell {
    /// Bash shell completion script generation
    Bash,
    /// Zsh shell completion script generation
    Zsh,
    /// Fish shell completion script generation
    Fish,
    /// PowerShell completion script generation
    Powershell,
    /// Elvish shell completion script generation
    Elvish,
}

#[cfg(feature = "completions")]
impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
            Shell::Powershell => "powershell",
            Shell::Elvish => "elvish",
        };
        write!(f, "{s}")
    }
}

fn validate_edition(val: &str) -> Result<String, String> {
    match val {
        "2015" | "2018" | "2021" | "2024" => Ok(val.to_string()),
        _ => Err("Invalid edition: must be one of 2015, 2018, 2021, 2024".to_string()),
    }
}

fn validate_license(val: &str) -> Result<String, String> {
    match val {
        "MIT" | "Apache-2.0" | "MIT OR Apache-2.0" => Ok(val.to_string()),
        _ => Err("Invalid license: must be MIT, Apache-2.0, or MIT OR Apache-2.0".to_string()),
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use pretty_assertions::assert_eq;

    #[test]
    fn verify_cli() {
        // This ensures the CLI will exit with an error if invalid arguments are provided
        Cli::command().debug_assert();
    }

    #[test]
    fn test_new_command() {
        let cli = Cli::parse_from(["cargo-quickstart", "new", "my-project", "--bin"]);
        match cli.command {
            Commands::New(new_args) => {
                assert_eq!(new_args.name, "my-project");
                assert!(new_args.bin);
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_init_command() {
        let cli = Cli::parse_from(["cargo-quickstart", "init", "--name", "my-lib", "--lib"]);
        match cli.command {
            Commands::Init(init_args) => {
                assert_eq!(init_args.name, Some("my-lib".to_string()));
                assert!(!init_args.bin);
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_init_with_custom_values() {
        let args = [
            "cargo-quickstart",
            "init",
            "--edition",
            "2018",
            "--license",
            "MIT",
            "--path",
            "/tmp/myproject",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Init(init_args) => {
                assert_eq!(init_args.edition, "2018");
                assert_eq!(init_args.license, "MIT");
                assert_eq!(init_args.path, PathBuf::from("/tmp/myproject"));
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_new_with_custom_path() {
        let args = [
            "cargo-quickstart",
            "new",
            "custom-project",
            "--path",
            "/tmp/projects",
            "--lib",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::New(new_args) => {
                assert_eq!(new_args.name, "custom-project");
                assert!(!new_args.bin);
                assert!(new_args.lib);
                assert_eq!(new_args.path, Some(PathBuf::from("/tmp/projects")));
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_list_templates_command() {
        let cli = Cli::parse_from(["cargo-quickstart", "list-templates"]);
        match cli.command {
            Commands::ListTemplates => {
                // Command parsed correctly
            }
            _ => panic!("Expected ListTemplates command"),
        }
    }

    #[test]
    fn test_list_templates_alias() {
        let cli = Cli::parse_from(["cargo-quickstart", "ls"]);
        match cli.command {
            Commands::ListTemplates => {
                // Command alias parsed correctly
            }
            _ => panic!("Expected ListTemplates command"),
        }
    }

    #[cfg(feature = "doctor")]
    #[test]
    fn test_doctor_command() {
        let cli = Cli::parse_from(["cargo-quickstart", "doctor"]);
        match cli.command {
            Commands::Doctor => {
                // Command parsed correctly
            }
            _ => panic!("Expected Doctor command"),
        }
    }

    #[cfg(feature = "completions")]
    #[test]
    fn test_completions_command() {
        let cli = Cli::parse_from(["cargo-quickstart", "completions", "bash"]);
        match cli.command {
            Commands::Completions(args) => {
                let shell_str = args.shell.to_string();
                assert_eq!(shell_str, "bash");
                assert_eq!(args.output, None);
            }
            _ => panic!("Expected Completions command"),
        }
    }

    #[cfg(feature = "completions")]
    #[test]
    fn test_completions_with_output() {
        let cli = Cli::parse_from([
            "cargo-quickstart",
            "completions",
            "zsh",
            "--output",
            "/tmp/completions.zsh",
        ]);
        match cli.command {
            Commands::Completions(args) => {
                let shell_str = args.shell.to_string();
                assert_eq!(shell_str, "zsh");
                assert_eq!(args.output, Some(PathBuf::from("/tmp/completions.zsh")));
            }
            _ => panic!("Expected Completions command"),
        }
    }

    #[cfg(feature = "completions")]
    #[test]
    fn test_completions_all_shells() {
        // Test each shell type can be parsed correctly
        let shells = vec![
            ("bash", "bash"),
            ("zsh", "zsh"),
            ("fish", "fish"),
            ("powershell", "powershell"),
            ("elvish", "elvish"),
        ];

        for (shell_arg, expected_str) in shells {
            let cli = Cli::parse_from(["cargo-quickstart", "completions", shell_arg]);
            match cli.command {
                Commands::Completions(args) => {
                    let shell_str = args.shell.to_string();
                    assert_eq!(shell_str, expected_str);
                }
                _ => panic!("Expected Completions command"),
            }
        }
    }

    #[test]
    fn test_validate_edition_valid() {
        assert_eq!(validate_edition("2015").unwrap(), "2015");
        assert_eq!(validate_edition("2018").unwrap(), "2018");
        assert_eq!(validate_edition("2021").unwrap(), "2021");
        assert_eq!(validate_edition("2024").unwrap(), "2024");
    }

    #[test]
    fn test_validate_edition_invalid() {
        assert!(validate_edition("2014").is_err());
        assert!(validate_edition("2025").is_err());
        assert!(validate_edition("nonexistent").is_err());
    }

    #[test]
    fn test_validate_license_valid() {
        assert_eq!(validate_license("MIT").unwrap(), "MIT");
        assert_eq!(validate_license("Apache-2.0").unwrap(), "Apache-2.0");
        assert_eq!(
            validate_license("MIT OR Apache-2.0").unwrap(),
            "MIT OR Apache-2.0"
        );
    }

    #[test]
    fn test_validate_license_invalid() {
        assert!(validate_license("GPL").is_err());
        assert!(validate_license("BSD").is_err());
        assert!(validate_license("invalid").is_err());
    }
}
