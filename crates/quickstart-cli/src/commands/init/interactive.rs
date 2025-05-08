//! Enhanced interactive wizard for project setup

use std::path::PathBuf;

use color_eyre::eyre::Report;
use color_eyre::Result;
use quickstart_lib::config::QuickstartConfig;
use quickstart_lib::ProjectType;

use crate::commands::init::inquire_api::{InquireApi, RealInquire};

/// Run the interactive setup wizard
pub fn run_wizard(path: PathBuf) -> Result<QuickstartConfig> {
    let inquire_api = RealInquire;
    run_wizard_with_api(&inquire_api, path)
}

/// Run the interactive setup wizard with dependency injection
/// This function allows for testing with a mock implementation
pub fn run_wizard_with_api<T: InquireApi>(
    inquire_api: &T,
    path: PathBuf,
) -> Result<QuickstartConfig> {
    // Check if directory exists and create it if needed
    if !path.exists() {
        let dir_str = path.display().to_string();
        let create_dir = inquire_api.confirm(
            &format!("Directory {dir_str} does not exist. Create it?"),
            true,
        )?;

        if !create_dir {
            return Err(Report::msg("Project creation cancelled"));
        }

        std::fs::create_dir_all(&path)
            .map_err(|e| Report::msg(format!("Failed to create directory: {e}")))?;
    }

    // Step 1: Project name
    let name = get_project_name_with_api(inquire_api)?;

    // Step 2: Project type (binary/library)
    let project_type = get_project_type_with_api(inquire_api)?;

    // Step 3: Rust edition
    let edition = get_rust_edition_with_api(inquire_api)?;

    // Step 4: License
    let license = get_license_with_api(inquire_api)?;

    // Step 5: Git initialization
    let git = get_git_init_with_api(inquire_api)?;

    // Step 6: Optional features
    let features = get_optional_features_with_api(inquire_api)?;

    // Create project config
    let config = QuickstartConfig {
        name,
        project_type,
        edition,
        license,
        git,
        path: path.clone(),
        yes: false,
        description: None,
        author: None,
        features: Some(features),
        plugins: None,
        dry_run: false,
        template_variant: None,
    };

    // Show summary and confirmation
    let mut summary = String::new();
    summary.push_str(&format!("Project name: {}\n", config.name));
    summary.push_str(&format!("Type: {}\n", config.project_type));
    summary.push_str(&format!("Edition: {}\n", config.edition));
    summary.push_str(&format!("License: {}\n", config.license));
    summary.push_str(&format!(
        "Initialize Git: {}\n",
        if config.git { "Yes" } else { "No" }
    ));

    if let Some(ref features) = config.features {
        summary.push_str("Additional features:\n");
        for feature in features {
            summary.push_str(&format!("  - {feature}\n"));
        }
    }

    println!("\nProject Summary:\n{summary}");

    let confirm = inquire_api.confirm("Create project with these settings?", true)?;

    if !confirm {
        return Err(Report::msg("Project creation cancelled by user"));
    }

    Ok(config)
}

/// Gets the project name from the user
#[allow(dead_code)]
pub fn get_project_name() -> Result<String> {
    let inquire_api = RealInquire;
    get_project_name_with_api(&inquire_api)
}

/// Gets the project name from the user with dependency injection
pub fn get_project_name_with_api<T: InquireApi>(inquire_api: &T) -> Result<String> {
    let validator = |input: &str| -> Result<bool, String> {
        if !input.trim().is_empty() && !input.trim().starts_with(|c: char| c.is_ascii_digit()) {
            Ok(true)
        } else {
            Ok(false)
        }
    };

    inquire_api.text_with_validation(
        "Project name:",
        Some("The name of your Rust project (valid crate name)"),
        validator,
        "Project name cannot be empty or start with a digit",
    )
}

/// Gets the project type from the user
#[allow(dead_code)]
pub fn get_project_type() -> Result<ProjectType> {
    let inquire_api = RealInquire;
    get_project_type_with_api(&inquire_api)
}

/// Gets the project type from the user with dependency injection
pub fn get_project_type_with_api<T: InquireApi>(inquire_api: &T) -> Result<ProjectType> {
    let options = ["Binary (application)", "Library"];

    match inquire_api.select("Project type:", &options, None)? {
        0 => Ok(ProjectType::Binary),
        1 => Ok(ProjectType::Library),
        _ => Ok(ProjectType::Binary), // Default to binary if somehow an invalid option is selected
    }
}

/// Gets the Rust edition from the user
#[allow(dead_code)]
pub fn get_rust_edition() -> Result<String> {
    let inquire_api = RealInquire;
    get_rust_edition_with_api(&inquire_api)
}

/// Gets the Rust edition from the user with dependency injection
pub fn get_rust_edition_with_api<T: InquireApi>(inquire_api: &T) -> Result<String> {
    let options = ["2021", "2018", "2015"];

    let idx = inquire_api.select(
        "Rust edition:",
        &options,
        Some("The Rust edition to use for your project"),
    )?;

    Ok(options[idx].to_string())
}

/// Gets the license from the user
#[allow(dead_code)]
pub fn get_license() -> Result<String> {
    let inquire_api = RealInquire;
    get_license_with_api(&inquire_api)
}

/// Gets the license from the user with dependency injection
pub fn get_license_with_api<T: InquireApi>(inquire_api: &T) -> Result<String> {
    let options = [
        "MIT OR Apache-2.0",
        "MIT",
        "Apache-2.0",
        "GPL-3.0",
        "BSD-3-Clause",
        "Unlicense",
        "Custom",
    ];

    let idx = inquire_api.select(
        "License:",
        &options,
        Some("The license to use for your project"),
    )?;

    let selection = options[idx];

    if selection == "Custom" {
        inquire_api.text(
            "Enter custom license identifier:",
            Some("Enter a valid SPDX license identifier"),
        )
    } else {
        Ok(selection.to_string())
    }
}

/// Gets git initialization preference from the user
#[allow(dead_code)]
pub fn get_git_init() -> Result<bool> {
    let inquire_api = RealInquire;
    get_git_init_with_api(&inquire_api)
}

/// Gets git initialization preference from the user with dependency injection
pub fn get_git_init_with_api<T: InquireApi>(inquire_api: &T) -> Result<bool> {
    inquire_api.confirm("Initialize Git repository?", true)
}

/// Gets optional features to include
#[allow(dead_code)]
pub fn get_optional_features() -> Result<Vec<String>> {
    let inquire_api = RealInquire;
    get_optional_features_with_api(&inquire_api)
}

/// Gets optional features to include with dependency injection
pub fn get_optional_features_with_api<T: InquireApi>(inquire_api: &T) -> Result<Vec<String>> {
    let features = [
        "README.md",
        ".gitignore",
        "CONTRIBUTING.md",
        "CI configuration",
        "VS Code configuration",
        "benchmarks",
        "examples",
    ];

    let indices = inquire_api.multiselect(
        "Select optional features:",
        &features,
        &[0, 1], // Default to README and .gitignore
        Some("Select additional files and features to include"),
    )?;

    Ok(indices
        .into_iter()
        .map(|i| features[i].to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::init::inquire_api::TestInquire;
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    #[test]
    fn test_get_project_name_valid() {
        let test_inquire = TestInquire::new();
        test_inquire.add_text("valid-project");

        let result = get_project_name_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(name) = result {
            assert_eq!(name, "valid-project");
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_get_project_name_invalid() {
        let test_inquire = TestInquire::new();
        test_inquire.add_text("");

        let result = get_project_name_with_api(&test_inquire);

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Validation failed"));
        } else {
            panic!("Result should be Err but was Ok");
        }
    }

    #[test]
    fn test_get_project_type_binary() {
        let test_inquire = TestInquire::new();
        test_inquire.add_select(0);

        let result = get_project_type_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(project_type) = result {
            assert_eq!(project_type, ProjectType::Binary);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_get_project_type_library() {
        let test_inquire = TestInquire::new();
        test_inquire.add_select(1);

        let result = get_project_type_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(project_type) = result {
            assert_eq!(project_type, ProjectType::Library);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_get_rust_edition() {
        let test_inquire = TestInquire::new();
        test_inquire.add_select(0);

        let result = get_rust_edition_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(edition) = result {
            assert_eq!(edition, "2021");
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_get_license_standard() {
        let test_inquire = TestInquire::new();
        test_inquire.add_select(1);

        let result = get_license_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(license) = result {
            assert_eq!(license, "MIT");
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_get_license_custom() {
        let test_inquire = TestInquire::new();
        test_inquire.add_select(6); // "Custom" option
        test_inquire.add_text("My-Custom-License");

        let result = get_license_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(license) = result {
            assert_eq!(license, "My-Custom-License");
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_get_optional_features() {
        let test_inquire = TestInquire::new();
        test_inquire.add_multiselect(vec![0, 2, 4]); // Selected features at indices 0, 2, and 4

        let result = get_optional_features_with_api(&test_inquire);

        assert!(result.is_ok());
        if let Ok(features) = result {
            assert_eq!(features.len(), 3);
            assert!(features.contains(&"README.md".to_string()));
            assert!(features.contains(&"CONTRIBUTING.md".to_string()));
            assert!(features.contains(&"VS Code configuration".to_string()));
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_run_wizard_complete_flow() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let test_inquire = TestInquire::new();

        // Set up mock responses for all prompts in order
        // 1. Project name
        test_inquire.add_text("test-project");
        // 2. Project type (Binary)
        test_inquire.add_select(0);
        // 3. Rust edition (2021)
        test_inquire.add_select(0);
        // 4. License (MIT)
        test_inquire.add_select(1);
        // 5. Git init (yes)
        test_inquire.add_confirm(true);
        // 6. Optional features (README + CI)
        test_inquire.add_multiselect(vec![0, 3]);
        // 7. Final confirmation
        test_inquire.add_confirm(true);

        let temp_dir = TempDir::new()?;
        let result = run_wizard_with_api(&test_inquire, temp_dir.path().to_path_buf());

        assert!(result.is_ok());
        if let Ok(config) = result {
            assert_eq!(config.name, "test-project");
            assert_eq!(config.project_type, ProjectType::Binary);
            assert_eq!(config.edition, "2021");
            assert_eq!(config.license, "MIT");
            assert!(config.git);
        } else {
            panic!("Result should be Ok but was Err");
        }
        Ok(())
    }

    #[test]
    fn test_run_wizard_cancel_at_end() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let test_inquire = TestInquire::new();

        // Set up all responses but cancel at the end
        test_inquire.add_text("test-project");
        test_inquire.add_select(0);
        test_inquire.add_select(0);
        test_inquire.add_select(1);
        test_inquire.add_confirm(true);
        test_inquire.add_multiselect(vec![0, 3]);
        test_inquire.add_confirm(false); // Cancel at final confirmation

        let temp_dir = TempDir::new()?;
        let result = run_wizard_with_api(&test_inquire, temp_dir.path().to_path_buf());

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Project creation cancelled by user");
        } else {
            panic!("Result should be Err but was Ok");
        }
        Ok(())
    }

    #[test]
    fn test_run_wizard_nonexistent_dir_create() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let test_inquire = TestInquire::new();

        // Confirm directory creation
        test_inquire.add_confirm(true);
        // Rest of the wizard prompts
        test_inquire.add_text("test-project");
        test_inquire.add_select(0);
        test_inquire.add_select(0);
        test_inquire.add_select(1);
        test_inquire.add_confirm(true);
        test_inquire.add_multiselect(vec![0]);
        test_inquire.add_confirm(true);

        let temp_dir = TempDir::new()?;
        let nonexistent_dir = temp_dir.path().join("nonexistent");
        let result = run_wizard_with_api(&test_inquire, nonexistent_dir.clone());

        assert!(result.is_ok());
        assert!(nonexistent_dir.exists());
        Ok(())
    }

    #[test]
    fn test_run_wizard_nonexistent_dir_cancel() -> Result<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let test_inquire = TestInquire::new();

        // Decline directory creation
        test_inquire.add_confirm(false);

        let temp_dir = TempDir::new()?;
        let nonexistent_dir = temp_dir.path().join("nonexistent");
        let result = run_wizard_with_api(&test_inquire, nonexistent_dir.clone());

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Project creation cancelled");
        } else {
            panic!("Result should be Err but was Ok");
        }
        assert!(!nonexistent_dir.exists());
        Ok(())
    }
}
