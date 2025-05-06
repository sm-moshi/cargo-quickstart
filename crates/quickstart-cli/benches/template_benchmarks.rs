//! Benchmarks for template-related functionality in cargo-quickstart
//!
//! These benchmarks measure the performance of template loading, listing, and rendering
//! operations to ensure optimum performance as templates evolve.

// Allow specific lints for benchmarks only
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quickstart_lib::{
    template::{TemplateLoader, TemplateVariant},
    ProjectConfig, ProjectType,
};
use std::{env, path::PathBuf};

/// Get the workspace root directory
#[allow(dead_code)]
fn workspace_root() -> PathBuf {
    // The bench is run from crates/quickstart-cli/,
    // so we need to go up two directories to get to the workspace root
    let current_dir = env::current_dir().expect("Failed to get current directory");

    if current_dir.ends_with("crates/quickstart-cli") {
        current_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    } else if current_dir.ends_with("quickstart-cli") {
        current_dir.parent().unwrap().to_path_buf()
    } else {
        // Assume we're already at the workspace root
        current_dir
    }
}

/// Benchmark template listing performance
fn benchmark_template_listing(c: &mut Criterion) {
    // Find the templates directory
    let template_dir = match quickstart_lib::find_templates_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error finding templates directory: {e}");
            return;
        }
    };

    let loader = TemplateLoader::new(template_dir);

    c.bench_function("list_templates", |b| {
        b.iter(|| {
            // Use black_box to prevent compiler optimizations from skewing results
            match black_box(loader.list_templates(ProjectType::Binary, TemplateVariant::Minimal)) {
                Ok(templates) => templates,
                Err(e) => {
                    panic!("Failed to list templates: {e}");
                }
            }
        })
    });
}

/// Set up a project config for template use
fn setup_project_config() -> ProjectConfig {
    ProjectConfig {
        name: "benchmark-project".to_string(),
        project_type: ProjectType::Binary,
        edition: "2021".to_string(),
        license: "MIT".to_string(),
        git: false,
        path: PathBuf::from("/tmp/bench"),
        yes: true,
    }
}

/// Set up and render templates that we know are available
fn benchmark_template_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("Template Rendering");

    // Find the templates directory
    let template_dir = match quickstart_lib::find_templates_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error finding templates directory: {e}");
            return;
        }
    };

    let loader = TemplateLoader::new(template_dir);
    let _config = setup_project_config();

    // Benchmark the README template which should be in all projects
    group.bench_function("common_readme", |b| {
        b.iter(|| {
            let template_path = "base/README.md.hbs";
            match black_box(loader.load_template(template_path)) {
                Ok(template) => template,
                Err(e) => panic!("Failed to load template '{template_path}': {e}"),
            }
        })
    });

    // Benchmark the Cargo.toml template which should be in all projects
    group.bench_function("common_cargo_toml", |b| {
        b.iter(|| {
            let template_path = "base/Cargo.toml.hbs";
            match black_box(loader.load_template(template_path)) {
                Ok(template) => template,
                Err(e) => panic!("Failed to load template '{template_path}': {e}"),
            }
        })
    });

    // Benchmark binary-specific template
    group.bench_function("binary_main", |b| {
        b.iter(|| {
            let template_path = "binary/minimal/src/main.rs.hbs";
            match black_box(loader.load_template(template_path)) {
                Ok(template) => template,
                Err(e) => panic!("Failed to load template '{template_path}': {e}"),
            }
        })
    });

    group.finish();
}

criterion_group!(
    name = template_benches;
    config = Criterion::default().sample_size(20);
    targets = benchmark_template_listing, benchmark_template_rendering
);
criterion_main!(template_benches);
