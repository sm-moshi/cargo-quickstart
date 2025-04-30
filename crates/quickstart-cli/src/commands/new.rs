//! Implementation of the 'new' command for creating a new project

#[allow(unused_imports)]
use color_eyre::{eyre::Report, Result};
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
