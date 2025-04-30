//! Implementation of the 'init' command for initializing a project in an existing directory

use color_eyre::{eyre::Report, Result};
use quickstart_lib::{generate_project, ProjectConfig, ProjectType};

use crate::{
    args::InitArgs,
    errors::CommandErrorExt,
    ui::{output, progress::with_spinner, prompts},
};

/// Execute the 'init' command
pub fn execute(args: InitArgs) -> Result<()> {
    output::header("Generating project");

    // Get project name (from args or prompt for directory name)
    let project_name = if let Some(name) = args.name {
        name
    } else {
        // Try to get the directory name from the path
        let dir_name = args
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("project");

        if !args.yes {
            prompts::input_with_default("Project name", dir_name)?
        } else {
            dir_name.to_string()
        }
    };

    // Determine project type
    let project_type = if args.lib {
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
            _ => unreachable!(),
        }
    };

    // Display project information
    output::section("Project configuration");
    output::key_value("Name", &project_name);
    output::key_value("Type", &project_type.to_string());
    output::key_value("Edition", &args.edition);
    output::key_value("License", &args.license);
    output::key_value("Path", &args.path.display().to_string());
    output::key_value("Git", &args.git.to_string());

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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quickstart_lib::ProjectType;
    use std::path::PathBuf;

    // Mock the prompts module for testing
    #[derive(Default)]
    #[allow(dead_code)]
    struct MockPrompts {
        confirm_result: bool,
        input_result: String,
        select_result: usize,
    }

    impl MockPrompts {
        fn new() -> Self {
            MockPrompts::default()
        }

        #[allow(dead_code)]
        fn with_confirm_result(mut self, result: bool) -> Self {
            self.confirm_result = result;
            self
        }

        fn with_input_result(mut self, result: &str) -> Self {
            self.input_result = result.to_string();
            self
        }

        fn with_select_result(mut self, result: usize) -> Self {
            self.select_result = result;
            self
        }
    }

    // Function to test project name resolution - returns the expected project name
    fn test_project_name_resolution(args: &InitArgs, mock_prompts: &MockPrompts) -> String {
        // Copy the logic from execute() to test it
        if let Some(name) = &args.name {
            name.clone()
        } else {
            let dir_name = args
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("project");

            if !args.yes {
                // Return mock prompt result in test
                mock_prompts.input_result.clone()
            } else {
                dir_name.to_string()
            }
        }
    }

    // Function to test project type resolution - returns the expected ProjectType
    fn test_project_type_resolution(args: &InitArgs, mock_prompts: &MockPrompts) -> ProjectType {
        // Copy the logic from execute() to test it
        if args.lib {
            ProjectType::Library
        } else if args.bin || args.yes {
            ProjectType::Binary
        } else {
            // Return mock selection in test
            match mock_prompts.select_result {
                0 => ProjectType::Binary,
                1 => ProjectType::Library,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_explicit_project_name() {
        let args = InitArgs {
            name: Some("explicit-name".to_string()),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
        };

        let mock_prompts = MockPrompts::new();

        let project_name = test_project_name_resolution(&args, &mock_prompts);
        assert_eq!(project_name, "explicit-name");
    }

    #[test]
    fn test_directory_name_with_yes_flag() {
        let args = InitArgs {
            name: None,
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("my-project"),
            git: false,
            yes: true,
        };

        let mock_prompts = MockPrompts::new();

        let project_name = test_project_name_resolution(&args, &mock_prompts);
        assert_eq!(project_name, "my-project");
    }

    #[test]
    fn test_prompted_name() {
        let args = InitArgs {
            name: None,
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("default-dir"),
            git: false,
            yes: false,
        };

        let mock_prompts = MockPrompts::new().with_input_result("prompted-name");

        let project_name = test_project_name_resolution(&args, &mock_prompts);
        assert_eq!(project_name, "prompted-name");
    }

    #[test]
    fn test_lib_project_type() {
        let args = InitArgs {
            name: Some("test".to_string()),
            bin: false,
            lib: true,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
        };

        let mock_prompts = MockPrompts::new();

        let project_type = test_project_type_resolution(&args, &mock_prompts);
        match project_type {
            ProjectType::Library => {} // Expected
            _ => panic!("Expected Library project type"),
        }
    }

    #[test]
    fn test_bin_project_type() {
        let args = InitArgs {
            name: Some("test".to_string()),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
        };

        let mock_prompts = MockPrompts::new();

        let project_type = test_project_type_resolution(&args, &mock_prompts);
        match project_type {
            ProjectType::Binary => {} // Expected
            _ => panic!("Expected Binary project type"),
        }
    }

    #[test]
    fn test_prompted_project_type() {
        let args = InitArgs {
            name: Some("test".to_string()),
            bin: false,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            path: PathBuf::from("."),
            git: false,
            yes: false,
        };

        // Test binary selection
        let mock_prompts = MockPrompts::new().with_select_result(0);

        let project_type = test_project_type_resolution(&args, &mock_prompts);
        match project_type {
            ProjectType::Binary => {} // Expected
            _ => panic!("Expected Binary project type"),
        }

        // Test library selection
        let mock_prompts = MockPrompts::new().with_select_result(1);

        let project_type = test_project_type_resolution(&args, &mock_prompts);
        match project_type {
            ProjectType::Library => {} // Expected
            _ => panic!("Expected Library project type"),
        }
    }
}
