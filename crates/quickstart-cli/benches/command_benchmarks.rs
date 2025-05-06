//! Benchmarks for command execution in cargo-quickstart
//!
//! These benchmarks measure the performance of various commands to ensure
//! the CLI provides a responsive and efficient user experience.

// Allow specific lints for benchmarks only
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{env, fs, path::PathBuf, process::Command};
use tempfile::TempDir;

/// Find the cargo-quickstart executable - build it if needed
fn prepare_executable() -> PathBuf {
    // Build the executable first to ensure it exists
    println!("Building cargo-quickstart executable for benchmarks...");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(workspace_root())
        .status()
        .expect("Failed to build cargo-quickstart");

    if !status.success() {
        panic!("Failed to build cargo-quickstart");
    }

    // Get the absolute path to the executable
    let exe_path = workspace_root().join("target/release/cargo-quickstart");

    if !exe_path.exists() {
        panic!(
            "Could not find cargo-quickstart executable at: {}",
            exe_path.display()
        );
    }

    println!("Using cargo-quickstart at: {}", exe_path.display());
    exe_path
}

/// Get the workspace root directory
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

/// Record the time to run a command without testing its output
fn benchmark_basic_commands(c: &mut Criterion) {
    let mut group = c.benchmark_group("Basic Commands");

    // Get the executable path
    let exe_path = prepare_executable();

    group.bench_function("list_templates", |b| {
        b.iter(|| {
            let output = Command::new(&exe_path)
                .args(["list-templates"])
                .output()
                .expect("Failed to execute command");

            // Debug information when the command fails
            if !output.status.success() {
                eprintln!("Command failed with status: {:?}", output.status);
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
            }

            assert!(output.status.success(), "Command failed");
            black_box(output);
        })
    });

    group.finish();
}

/// Create a basic project configuration for testing
fn benchmark_project_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Project Creation");

    // Get the executable path
    let exe_path = prepare_executable();

    // Test different project types
    for project_type in ["--bin", "--lib"] {
        group.bench_function(
            format!(
                "new_{}",
                if project_type == "--bin" {
                    "binary"
                } else {
                    "library"
                }
            ),
            |b| {
                b.iter_with_setup(
                    // Setup: create a unique project name
                    || {
                        let unique_id = uuid::Uuid::new_v4().to_string();
                        let project_name = format!("bench_project_{}", unique_id);
                        (env::temp_dir(), project_name)
                    },
                    // Benchmark: create the project
                    |(temp_dir, project_name)| {
                        let project_path = temp_dir.join(&project_name);

                        // Clean up any existing directory
                        if project_path.exists() {
                            let _ = fs::remove_dir_all(&project_path);
                        }

                        // Create the project
                        let output = Command::new(&exe_path)
                            .args([
                                "new",
                                &project_name,
                                project_type,
                                "--path",
                                project_path.to_str().unwrap(),
                                "--yes",
                            ])
                            .output()
                            .expect("Failed to execute command");

                        // Debug information when the command fails
                        if !output.status.success() {
                            eprintln!("Command failed with status: {:?}", output.status);
                            eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                            eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                        }

                        // Clean up
                        let _ = fs::remove_dir_all(project_path);

                        assert!(output.status.success(), "Command failed");
                        black_box(output);
                    },
                );
            },
        );
    }

    group.finish();
}

/// Benchmark project initialization in an existing directory
fn benchmark_project_init(c: &mut Criterion) {
    let mut group = c.benchmark_group("Project Init");

    // Get the executable path
    let exe_path = prepare_executable();

    // Map bin/lib to test function names
    let type_names = [("bin", "--bin"), ("lib", "--lib")];

    for (name, flag) in type_names {
        group.bench_function(format!("init_{}", name), |b| {
            b.iter_with_setup(
                // Setup: create a temporary directory
                || TempDir::new().expect("Failed to create temporary directory"),
                // Benchmark: initialize project
                |temp_dir| {
                    let path = temp_dir.path();

                    // Execute the init command
                    let output = Command::new(&exe_path)
                        .args([
                            "init",
                            flag,
                            "--path",
                            path.to_str().unwrap(),
                            "--name",
                            "bench-project",
                            "--yes",
                        ])
                        .output()
                        .expect("Failed to execute command");

                    // Debug information when the command fails
                    if !output.status.success() {
                        eprintln!("Command failed with status: {:?}", output.status);
                        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                    }

                    assert!(output.status.success(), "Command failed");
                    black_box(output);
                },
            );
        });
    }

    group.finish();
}

criterion_group!(
    name = command_benches;
    config = Criterion::default().sample_size(10).measurement_time(std::time::Duration::from_secs(10));
    targets = benchmark_basic_commands, benchmark_project_creation, benchmark_project_init
);
criterion_main!(command_benches);
