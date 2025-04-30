//! Command-line argument definitions for cargo-quickstart

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// A cargo subcommand for quickly generating opinionated Rust projects
#[derive(Parser, Debug)]
#[command(
    name = "cargo-quickstart",
    author,
    version,
    about,
    long_about = "A blazing fast and opinionated cargo subcommand for bootstrapping modern Rust projects with confidence and speed."
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
    #[command(name = "new", visible_alias = "n")]
    New(NewArgs),

    /// Initialize an existing directory with a Rust project
    #[command(name = "init", visible_alias = "i")]
    Init(InitArgs),
}

/// Arguments for the 'new' command
#[derive(Args, Debug)]
pub struct NewArgs {
    /// Name of the new project
    pub name: String,

    /// Create a binary application [default if neither --bin nor --lib is specified]
    #[arg(long, conflicts_with = "lib")]
    pub bin: bool,

    /// Create a library crate
    #[arg(long, conflicts_with = "bin")]
    pub lib: bool,

    /// Rust edition to use
    #[arg(long, default_value = "2021")]
    pub edition: String,

    /// License to apply (MIT, Apache-2.0, or dual MIT/Apache-2.0)
    #[arg(long, default_value = "MIT OR Apache-2.0")]
    pub license: String,

    /// Initialize a Git repository
    #[arg(long)]
    pub git: bool,

    /// Target directory (defaults to the project name in the current directory)
    #[arg(long)]
    pub path: Option<PathBuf>,

    /// Accept all defaults without prompting
    #[arg(short, long)]
    pub yes: bool,
}

/// Arguments for the 'init' command
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Create a binary application [default if neither --bin nor --lib is specified]
    #[arg(long, conflicts_with = "lib")]
    pub bin: bool,

    /// Create a library crate
    #[arg(long, conflicts_with = "bin")]
    pub lib: bool,

    /// Name of the project (defaults to directory name)
    #[arg(long)]
    pub name: Option<String>,

    /// Rust edition to use
    #[arg(long, default_value = "2021")]
    pub edition: String,

    /// License to apply (MIT, Apache-2.0, or dual MIT/Apache-2.0)
    #[arg(long, default_value = "MIT OR Apache-2.0")]
    pub license: String,

    /// Initialize a Git repository
    #[arg(long)]
    pub git: bool,

    /// Target directory (defaults to the current directory)
    #[arg(long, default_value = ".")]
    pub path: PathBuf,

    /// Accept all defaults without prompting
    #[arg(short, long)]
    pub yes: bool,
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
