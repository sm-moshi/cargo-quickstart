//! Project configuration handling for the init command

use color_eyre::Result;
use quickstart_lib::ProjectType;

use crate::{
    args::InitArgs,
    ui::{output, prompts},
};

/// Get project name from args or prompt user
pub fn get_project_name(args: &InitArgs) -> Result<String> {
    if let Some(name) = &args.name {
        Ok(name.clone())
    } else {
        // Try to get the directory name from the path
        let dir_name = args
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("project");

        if !args.yes {
            prompts::input_with_default("Project name", dir_name)
        } else {
            Ok(dir_name.to_string())
        }
    }
}

/// Determine project type based on args or user input
pub fn determine_project_type(args: &InitArgs) -> Result<ProjectType> {
    Ok(if args.lib {
        ProjectType::Library
    } else if args.bin || args.yes {
        ProjectType::Binary // Default to binary in non-interactive/bin mode
    } else {
        // Prompt for project type
        let options = &["Binary application", "Library crate"];
        let selection = prompts::select("Project type", options)?;

        match selection {
            0 => ProjectType::Binary,
            1 => ProjectType::Library,
            _ => {
                // Fallback to Binary if selection is somehow out of range
                output::warning("Unexpected project type selection, defaulting to Binary");
                ProjectType::Binary
            }
        }
    })
}

/// Display project configuration
pub fn display_project_info(project_name: &str, project_type: &ProjectType, args: &InitArgs) {
    output::section("Project configuration");
    output::key_value("Name", project_name);
    output::key_value("Type", &project_type.to_string());
    output::key_value("Edition", &args.edition);
    output::key_value("License", &args.license);
    output::key_value("Path", &args.path.display().to_string());
    output::key_value("Git", &args.git.to_string());
}
