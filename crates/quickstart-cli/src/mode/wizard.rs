//! Wizard mode implementations for cargo-quickstart
//!
//! This module provides the wizard mode implementation for cargo-quickstart.
//!
//! The wizard mode is a text-based user interface that allows the user to interact with the application.
//!
//! The wizard mode is currently under development and is not yet available.
//!
// TODO: Implement the wizard mode - THIS TODO IS BEING ADDRESSED NOW
//! Have quickstart-cli/src/ui/ in mind
//! Have docs/ROADMAP.md in mind

use crate::args;
use crate::errors::CliError;

use crate::ui::prompts;
use quickstart_lib::{
    config::{Author, QuickstartConfig, TemplateVariant},
    generate_project, ProjectType,
}; // Assuming generate_project is pub from quickstart_lib

use color_eyre::Result;
use std::path::PathBuf;

// Define a simple trait for common args accessed by the wizard
pub trait CommonArgs {
    fn get_yes(&self) -> bool;
}

impl CommonArgs for args::NewArgs {
    fn get_yes(&self) -> bool {
        self.yes
    }
}

impl CommonArgs for args::InitArgs {
    fn get_yes(&self) -> bool {
        self.yes
    }
}

pub fn run<T: CommonArgs>(common_args: &T, dry_run_flag: bool) -> Result<(), CliError> {
    println!("Welcome to the cargo-quickstart wizard!\n");

    // --- Project Name ---
    let project_name = prompts::project_name("Enter the project name:")
        .map_err(|e| CliError::InquireError(e.to_string()))?;

    let mut config = QuickstartConfig {
        name: project_name,
        ..Default::default()
    };

    // --- Project Path (derive default from name) ---
    let default_path_str = format!("./{}", config.name);
    let path_str = prompts::input_with_default("Project directory:", &default_path_str)
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    config.path = PathBuf::from(path_str);

    // --- Project Type ---
    let project_type_options = ["Binary", "Library"];
    let type_idx = prompts::select("Choose project type:", &project_type_options)
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    config.project_type = match project_type_options[type_idx] {
        "Binary" => ProjectType::Binary,
        "Library" => ProjectType::Library,
        _ => unreachable!(), // Should not happen with defined options
    };

    // --- Edition ---
    let edition_options = ["2021", "2024"]; // TODO: Consider making this dynamic or configurable
    let edition_idx = prompts::select("Select Rust edition:", &edition_options)
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    config.edition = edition_options[edition_idx].to_string();

    // --- License ---
    let license_options = ["MIT OR Apache-2.0", "MIT", "Apache-2.0", "None"];
    let license_idx = prompts::select("Choose a license:", &license_options)
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    config.license = if license_options[license_idx] == "None" {
        String::new() // Represent "None" as an empty string or handle as appropriate
    } else {
        license_options[license_idx].to_string()
    };

    // --- Initialize Git ---
    config.git = prompts::confirm("Initialize a Git repository?", true)
        .map_err(|e| CliError::InquireError(e.to_string()))?;

    // --- Description (Optional) ---
    let description = prompts::input_with_default("Project description (optional):", "")
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    if !description.is_empty() {
        config.description = Some(description);
    }

    // --- Author (Optional) ---
    let author_name = prompts::input_with_default("Author's name (optional):", "")
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    if !author_name.is_empty() {
        let author_email_str = prompts::input_with_default("Author's email (optional):", "")
            .map_err(|e| CliError::InquireError(e.to_string()))?;
        let email = if author_email_str.is_empty() {
            None
        } else {
            Some(author_email_str)
        };
        config.author = Some(Author {
            name: author_name,
            email,
        });
    }

    // --- Template Variant ---
    let template_variant_options = ["Extended", "Minimal"];
    let variant_idx = prompts::select("Choose template variant:", &template_variant_options)
        .map_err(|e| CliError::InquireError(e.to_string()))?;
    config.template_variant = Some(match template_variant_options[variant_idx] {
        "Extended" => TemplateVariant::Extended,
        "Minimal" => TemplateVariant::Minimal,
        _ => unreachable!(), // Should not happen
    });

    // Pass through CLI flags that don't have a direct wizard prompt
    config.yes = common_args.get_yes();
    config.dry_run = dry_run_flag;

    // --- Optional: Display a summary and ask for final confirmation ---
    // TODO: Consider adding a display of the config and a final confirmation step.
    // For now, proceed directly to generation.
    // Example:
    // println!("\nProject Configuration:\n{:#?}", config);
    // if !prompts::confirm("Proceed with generation?", true).map_err(|e| CliError::Inquire(e.to_string()))? {
    //     println!("Generation cancelled by user.");
    //     return Ok(());
    // }

    println!("\nConfiguration complete. Generating project...");

    // Call the project generation function from quickstart_lib
    generate_project(config).map_err(CliError::LibraryError)?;

    // Using println! for success message here; could be a more structured UI output
    println!(
        "\n🚀 Project '{}' scaffolded successfully!",
        prompts::project_name("Re-enter project name for confirmation message (temporary):")
            .unwrap_or_else(|_| "New Project".to_string()) // Fallback for prompt error
    );
    // TODO: The success message ideally uses config.name, but if prompts failed, config.name might be empty.
    // This re-prompt is a temporary measure. A better approach would be to ensure config.name is robustly set
    // or handle the case where it might not be available for the success message.

    Ok(())
}
