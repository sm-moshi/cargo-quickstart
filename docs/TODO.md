# TODO – cargo-quickstart

This TODO list tracks detailed and structured work needed to complete, refine, and expand `cargo-quickstart` based on the current project vision.

⸻

## Core CLI

**Goal:** Build a robust and ergonomic command-line interface.

-   [x] Define complete `clap` structure:
    -   [x] Implement root command with description and version
    -   [x] Add essential flags:
        -   [x] `--bin` (binary project)
        -   [x] `--lib` (library project)
        -   [x] `--edition` (2021/2024)
        -   [x] `--license` (MIT, Apache-2.0, etc.)
        -   [x] `--git` (initialize Git repository)
        -   [x] `--yes` (accept defaults)
        -   [x] `--path` (project directory)
        -   [x] `--interactive` (prompt-driven setup)
    -   [x] Implement input validation and graceful error handling
    -   [x] Polish CLI output formatting (`--help`, `--version`)

⸻

## Interactive UI

**Goal:** Create a user-friendly interactive experience.

-   [x] Implement dialoguer-based prompts:
    -   [x] Project name input with validation
    -   [x] Project type selection
    -   [x] Configuration confirmation
-   [x] Create progress indicators for long-running operations
-   [x] Design consistent message formatting with color support
-   [ ] Enhance interactive mode:
    -   [ ] Add validation for various input types
    -   [ ] Implement context-aware default values
    -   [ ] Create a complete wizard-style interface
-   [ ] Add error recovery suggestions in interactive mode
-   [ ] Implement accessibility considerations for terminal UI

⸻

## Template Engine

**Goal:** Create flexible, dynamic project scaffolding.

-   [x] Build structured `templates/` folder hierarchy:
    -   [x] Base templates (Cargo.toml, README.md, LICENSE)
    -   [x] Configuration templates (.vscode/, .cargo/, .editorconfig, .gitignore)
    -   [x] Documentation templates (CHANGELOG.md, CONTRIBUTING.md, etc.)
-   [x] Add dynamic template interpolation:
    -   [x] Replace variables like `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`, `{description}`
-   [x] Offer Minimal and Extended template variants
-   [x] Design a system for template versioning and future expansion (initial version in place)
-   [x] Template system is robust, fully tested, and production-ready as of MVP

⸻

## Generator Logic

**Goal:** Automate high-quality project generation end-to-end.

-   [x] Create ProjectConfig type for consistent data handling
-   [x] Implement project type enum (Binary/Library)
-   [x] Scaffold directory structure safely and cleanly (template-based)
-   [x] Implement optional Git repository initialization:
    -   [ ] `git init`
    -   [ ] Create first commit if applicable
-   [x] Generate detailed Cargo.toml:
    -   [ ] Profiles (dev, release, profiling)
    -   [ ] Workspace metadata
-   [x] Generate `.cargo/config.toml` with sccache settings
-   [x] Scaffold full `.vscode/` integration:
    -   [ ] settings.json
    -   [ ] tasks.json
    -   [ ] launch.json
    -   [ ] extensions.json
-   [x] Generate clean and readable Justfile
-   [x] Generate LICENSE file(s) based on user input
-   [x] Implement developer tool installer (ensure-tools)
-   [ ] Add `--dry-run` mode (preview changes without filesystem writes)

---

**Template system MVP is complete and fully integrated.**

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

-   [x] Write unit tests for CLI parsing and validation
-   [x] Write integration tests for end-to-end project generation
-   [x] Create test fixtures for validation scenarios
-   [x] Implement mock testing for interactive components
-   [x] Set up coverage reporting
-   [ ] Improve testing of interactive components:
    -   [ ] Add automated tests for dialoguer components
    -   [ ] Create more robust mocks for user input
    -   [ ] Test edge cases in user interaction
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
