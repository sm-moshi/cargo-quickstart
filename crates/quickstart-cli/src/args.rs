//! Command-line argument definitions for cargo-quickstart

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// A cargo subcommand for quickly generating opinionated Rust projects
#[derive(Parser, Debug)]
#[command(
    name = "cargo-quickstart",
    author,
    version,
    about = "Opinionated cargo subcommand for bootstrapping modern Rust projects.",
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
    #[command(
        name = "completions",
        about = "Generate shell completion scripts for your shell (bash, zsh, fish, powershell, elvish)",
        long_about = "Generate shell completion scripts for your shell. Example: cargo quickstart completions bash > /usr/local/etc/bash_completion.d/cargo-quickstart"
    )]
    Completions(CompletionsArgs),
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
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
    Elvish,
}

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
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        // This ensures the CLI will exit with an error if invalid arguments are provided
        Cli::command().debug_assert();
    }

    #[test]
    fn test_new_command_parsing() {
        let args = [
            "cargo-quickstart",
            "new",
            "my-project",
            "--bin",
            "--git",
            "--yes",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::New(new_args) => {
                assert_eq!(new_args.name, "my-project");
                assert!(new_args.bin);
                assert!(!new_args.lib);
                assert_eq!(new_args.edition, "2021"); // Default value
                assert_eq!(new_args.license, "MIT OR Apache-2.0"); // Default value
                assert!(new_args.git);
                assert!(new_args.yes);
                assert!(new_args.path.is_none());
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_init_command_parsing() {
        let args = [
            "cargo-quickstart",
            "init",
            "--lib",
            "--name",
            "my-lib",
            "--git",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Init(init_args) => {
                assert_eq!(init_args.name, Some("my-lib".to_string()));
                assert!(!init_args.bin);
                assert!(init_args.lib);
                assert_eq!(init_args.edition, "2021"); // Default value
                assert_eq!(init_args.license, "MIT OR Apache-2.0"); // Default value
                assert!(init_args.git);
                assert!(!init_args.yes);
                assert_eq!(init_args.path, PathBuf::from(".")); // Default value
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
}
