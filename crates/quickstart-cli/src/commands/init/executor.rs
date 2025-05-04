//! Main execution logic for the init command

use color_eyre::{eyre::Report, Result};
use quickstart_lib::{generate_project, ProjectConfig};

use crate::{
    args::InitArgs,
    errors::CommandErrorExt,
    ui::{output, progress::with_spinner, prompts},
};

use super::config::{determine_project_type, display_project_info, get_project_name};

/// Execute the 'init' command
pub fn execute(args: InitArgs) -> Result<()> {
    output::header("Generating project");

    // Get project name and type
    let project_name = get_project_name(&args)?;
    let project_type = determine_project_type(&args)?;

    // Display project information
    display_project_info(&project_name, &project_type, &args);

    // Prompt for confirmation if not in yes mode
    if !args.yes {
        let confirmed = prompts::confirm("Initialize project with these settings?", true)?;
        if !confirmed {
            return Err(Report::msg("Project initialization cancelled by user"));
        }
    }

    // Build configuration
    let config = ProjectConfig {
        name: project_name,
        project_type,
        edition: args.edition,
        license: args.license,
        path: args.path,
        git: args.git,
        yes: args.yes,
    };

    // Generate project with a progress spinner
    with_spinner(
        "Initializing project...",
        "Project initialized successfully!",
        || generate_project(config).command_context("init"),
    )
}
