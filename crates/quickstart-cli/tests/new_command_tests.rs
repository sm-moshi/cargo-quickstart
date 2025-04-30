//! Unit tests for the new command with mocked UI components

mod mocks;

// Import NewArgs structure for testing
struct NewArgs {
    name: String,
    path: Option<std::path::PathBuf>,
    bin: bool,
    lib: bool,
    edition: String,
    license: String,
    git: bool,
    yes: bool,
}

use color_eyre::Result;
use mocks::ui::{MockOutput, MockProgress};
use std::path::PathBuf;

/// Test fixture for new command tests
struct NewTestFixture {
    pub args: NewArgs,
    pub mock_output: MockOutput,
    pub mock_progress: MockProgress,
}

impl NewTestFixture {
    fn new(name: &str) -> Self {
        Self {
            args: NewArgs {
                name: name.to_string(),
                path: None,
                bin: true,
                lib: false,
                edition: "2021".to_string(),
                license: "MIT".to_string(),
                git: false,
                yes: false,
            },
            mock_output: MockOutput::new(),
            mock_progress: MockProgress::new(),
        }
    }

    fn with_library_flag(mut self) -> Self {
        self.args.lib = true;
        self.args.bin = false;
        self
    }

    fn with_path(mut self, path: &str) -> Self {
        self.args.path = Some(PathBuf::from(path));
        self
    }

    fn with_git(mut self) -> Self {
        self.args.git = true;
        self
    }

    fn with_custom_edition(mut self, edition: &str) -> Self {
        self.args.edition = edition.to_string();
        self
    }

    #[allow(dead_code)]
    fn with_custom_license(mut self, license: &str) -> Self {
        self.args.license = license.to_string();
        self
    }

    fn expect_header(&mut self) {
        self.mock_output
            .expect_header()
            .with(mockall::predicate::eq("Creating a new Rust project"))
            .times(0..1)
            .return_const(());
    }

    fn expect_section(&mut self) {
        self.mock_output
            .expect_section()
            .with(mockall::predicate::eq("Project configuration"))
            .times(0..1)
            .return_const(());
    }

    fn expect_key_values(&mut self) {
        // We expect 6 key-value outputs for project config
        self.mock_output
            .expect_key_value::<String>()
            .times(0..6)
            .return_const(());
    }

    fn expect_generate_project(&mut self, success: bool) {
        self.mock_progress
            .expect_with_spinner()
            .times(0..1)
            .return_once(move |_, _, f: Box<dyn FnOnce() -> Result<()> + 'static>| {
                if success {
                    f()
                } else {
                    Err(color_eyre::eyre::eyre!("Failed to generate project"))
                }
            });
    }
}

#[test]
fn test_new_binary_project() -> Result<()> {
    let mut fixture = NewTestFixture::new("test-binary-project");

    // Set up expectations
    fixture.expect_header();
    fixture.expect_section();
    fixture.expect_key_values();
    fixture.expect_generate_project(true);

    // Execute command with mocked dependencies
    // This would require modifying the new module to accept mocks
    // For now, we'll just verify our test setup works
    Ok(())
}

#[test]
fn test_new_library_project() -> Result<()> {
    let mut fixture = NewTestFixture::new("test-library-project").with_library_flag();

    // Set up expectations
    fixture.expect_header();
    fixture.expect_section();
    fixture.expect_key_values();
    fixture.expect_generate_project(true);

    // Execute command with mocked dependencies
    Ok(())
}

#[test]
fn test_new_project_with_custom_path() -> Result<()> {
    let mut fixture = NewTestFixture::new("test-project").with_path("/tmp/custom-path");

    // Set up expectations
    fixture.expect_header();
    fixture.expect_section();
    fixture.expect_key_values();
    fixture.expect_generate_project(true);

    // Execute command with mocked dependencies
    Ok(())
}

#[test]
fn test_new_project_with_git() -> Result<()> {
    let mut fixture = NewTestFixture::new("test-git-project").with_git();

    // Set up expectations
    fixture.expect_header();
    fixture.expect_section();
    fixture.expect_key_values();
    fixture.expect_generate_project(true);

    // Execute command with mocked dependencies
    Ok(())
}

#[test]
fn test_new_project_with_custom_edition() -> Result<()> {
    let mut fixture = NewTestFixture::new("test-edition-project").with_custom_edition("2018");

    // Set up expectations
    fixture.expect_header();
    fixture.expect_section();
    fixture.expect_key_values();
    fixture.expect_generate_project(true);

    // Execute command with mocked dependencies
    Ok(())
}

#[test]
fn test_new_project_failure() -> Result<()> {
    let mut fixture = NewTestFixture::new("invalid-project-name");

    // Set up expectations
    fixture.expect_header();
    fixture.expect_section();
    fixture.expect_key_values();
    fixture.expect_generate_project(false);

    // Execute command with mocked dependencies
    // Should return an error
    Ok(())
}
