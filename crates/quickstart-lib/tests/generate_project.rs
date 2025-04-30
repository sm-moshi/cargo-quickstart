//! Integration test for project generation logic

use quickstart_lib::{generate_project, ProjectConfig, ProjectType};
use std::path::PathBuf;

#[test]
fn generates_project_config_successfully() {
    let config = ProjectConfig {
        name: "example_project".into(),
        project_type: ProjectType::Binary,
        edition: "2021".into(),
        license: "MIT".into(),
        path: PathBuf::from("tmp/test-project"),
        git: false,
        yes: true,
    };

    let result = generate_project(config);
    assert!(result.is_ok());
}
