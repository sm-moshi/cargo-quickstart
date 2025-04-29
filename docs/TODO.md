# TODO – cargo-quickstart

This TODO list tracks detailed and structured work needed to complete, refine, and expand `cargo-quickstart` based on the current project vision.

⸻

## Core CLI

**Goal:** Build a robust and ergonomic command-line interface.

-   [ ] Define complete `clap` structure:
    -   [ ] Implement root command with description and version
    -   [ ] Add essential flags:
        -   [ ] `--bin` (binary project)
        -   [ ] `--lib` (library project)
        -   [ ] `--edition` (2021/2024)
        -   [ ] `--license` (MIT, Apache-2.0, etc.)
        -   [ ] `--git` (initialize Git repository)
        -   [ ] `--yes` (accept defaults)
        -   [ ] `--path` (project directory)
        -   [ ] `--interactive` (prompt-driven setup)
    -   [ ] Implement input validation and graceful error handling
    -   [ ] Polish CLI output formatting (`--help`, `--version`)

⸻

## Template Engine

**Goal:** Create flexible, dynamic project scaffolding.

-   [ ] Build structured `templates/` folder hierarchy:
    -   [ ] Base templates (Cargo.toml, README.md, LICENSE)
    -   [ ] Configuration templates (.vscode/, .cargo/, .editorconfig, .gitignore)
    -   [ ] Documentation templates (CHANGELOG.md, CONTRIBUTING.md, etc.)
-   [ ] Add dynamic template interpolation:
    -   [ ] Replace variables like `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`, `{description}`
-   [ ] Offer Minimal and Extended template variants
-   [ ] Design a system for template versioning and future expansion

⸻

## Generator Logic

**Goal:** Automate high-quality project generation end-to-end.

-   [ ] Scaffold directory structure safely and cleanly
-   [ ] Implement optional Git repository initialization:
    -   [ ] `git init`
    -   [ ] Create first commit if applicable
-   [ ] Generate detailed Cargo.toml:
    -   [ ] Profiles (dev, release, profiling)
    -   [ ] Workspace metadata
-   [ ] Generate `.cargo/config.toml` with sccache settings
-   [ ] Scaffold full `.vscode/` integration:
    -   [ ] settings.json
    -   [ ] tasks.json
    -   [ ] launch.json
    -   [ ] extensions.json
-   [ ] Generate clean and readable Justfile
-   [ ] Generate LICENSE file(s) based on user input
-   [ ] Implement developer tool installer (ensure-tools)
-   [ ] Add `--dry-run` mode (preview changes without filesystem writes)

⸻

## Developer Experience (DX) + Documentation

**Goal:** Make projects immediately developer- and contributor-friendly.

-   [ ] Scaffold standard project documentation:
    -   [ ] README.md (auto-filled with project metadata)
    -   [ ] CHANGELOG.md (initial entry)
    -   [ ] CONTRIBUTING.md (contributor guidelines)
    -   [ ] CODE_OF_CONDUCT.md (community standard)
    -   [ ] ROADMAP.md (future plans)
    -   [ ] TODO.md (task tracking)
-   [ ] Include root project configuration files:
    -   [ ] .editorconfig
    -   [ ] .gitignore
    -   [ ] rustfmt.toml
    -   [ ] clippy.toml
-   [ ] Validate that generated projects pass:
    -   [ ] `cargo check`
    -   [ ] `cargo fmt`
    -   [ ] `cargo clippy`

⸻

## Testing and Validation

**Goal:** Ensure all generated projects are reliable and production-grade.

-   [ ] Write unit tests for CLI parsing and validation
-   [ ] Write integration tests for end-to-end project generation
-   [ ] Scaffold mock project directories during tests
-   [ ] Validate that generated projects build successfully
-   [ ] Validate that formatting, linting, and testing pass:
    -   [ ] `cargo fmt`
    -   [ ] `cargo clippy`
    -   [ ] `cargo test`
    -   [ ] `cargo nextest`
-   [ ] Verify optional tooling is correctly installed

⸻

## CI / Publish

**Goal:** Maintain high build quality and automate release processes.

-   [ ] Set up GitHub Actions workflows:
    -   [ ] `cargo fmt` formatting check
    -   [ ] `cargo clippy` linting with denied warnings
    -   [ ] `cargo test` on stable, beta, nightly
-   [ ] Perform `cargo publish --dry-run` validation in CI
-   [ ] Integrate `cargo-release` or `cargo-smart-release` pipelines
-   [ ] Maintain Minimum Supported Rust Version (MSRV) tracking
-   [ ] Scaffold CI configuration templates into projects automatically

⸻

Let's make `cargo-quickstart` the fastest, cleanest, and most powerful way to start a modern Rust project!
