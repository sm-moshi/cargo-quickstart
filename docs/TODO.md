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
-   [ ] Add CLI mode handling:
    -   [ ] `--manual` → Manual mode
    -   [ ] `--interactive` → TUI mode
    -   [ ] Default = Wizard mode
-   [ ] Implement mode dispatcher in `main.rs`
-   [ ] Add template preset support:
    -   [ ] `--preset cli` for command-line tools
    -   [ ] `--preset lib` for API/library crates
    -   [ ] `--preset web` for Axum/Rocket web projects

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
-   [ ] UX enhancements:
    -   [ ] Create shared `QuickstartConfig` struct used across all modes
    -   [ ] Allow dry-run output preview
    -   [ ] Show post-run summary with generated paths

⸻

## Template Engine

**Goal:** Create flexible, dynamic project scaffolding.

-   [x] Build structured `templates/` folder hierarchy:
    -   [x] Base templates (Cargo.toml, README.md, LICENSE)
    -   [~] Configuration templates (.vscode/, .cargo/, .editorconfig, .gitignore)
    -   [x] Documentation templates (CHANGELOG.md, CONTRIBUTING.md, etc.)
-   [x] Add dynamic template interpolation:
    -   [x] Replace variables like `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`, `{description}`
-   [x] Offer Minimal and Extended template variants
-   [x] Design a system for template versioning and future expansion (initial version in place)
-   [x] Template system is robust, fully tested, and production-ready as of MVP
-   [ ] Add remote template support
-   [ ] Add template discovery and info system
-   [ ] Implement template registry
-   [ ] Tag template presets (`cli`, `lib`, `web`, `wasm`)
-   [ ] Generate `README.md`, `LICENSE`, and `.gitignore` files dynamically

⸻

## Performance Optimization

**Goal:** Enhance speed and resource efficiency across all operations.

-   [~] Template engine performance:
    -   [~] Implement template caching to avoid re-parsing templates
    -   [ ] Optimize Handlebars helper functions
    -   [ ] Consider specialized template engine for core use cases
-   [ ] File system operations:
    -   [ ] Implement parallel file operations using rayon
    -   [ ] Use batched file system operations where possible
    -   [ ] Minimize directory traversals with more efficient algorithms
-   [ ] Memory management:
    -   [ ] Replace String with Cow<'static, str> for template variables
    -   [ ] Implement string interning for repeated template variables
    -   [ ] Reduce allocations in hot paths
-   [ ] Build configuration:
    -   [ ] Apply profile-guided optimization for release builds
    -   [ ] Evaluate custom allocators (mimalloc/jemalloc)
    -   [ ] Fine-tune LTO and codegen-units settings
-   [x] Benchmarking infrastructure:
    -   [x] Criterion-based benchmarks for template operations
    -   [x] pprof integration for profiling and flamegraphs
    -   [x] just commands for performance testing

⸻

## Generator Logic

**Goal:** Automate high-quality project generation end-to-end.

-   [x] Create ProjectConfig type for consistent data handling
-   [x] Implement project type enum (Binary/Library)
-   [x] Scaffold directory structure safely and cleanly (template-based)
-   [x] Add path validation to ensure parent directories exist before attempting to create project directories
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

-   [x] Scaffold standard project documentation:
    -   [x] README.md (auto-filled with project metadata)
    -   [x] CHANGELOG.md (initial entry)
    -   [x] CONTRIBUTING.md (contributor guidelines)
    -   [x] CODE_OF_CONDUCT.md (community standard)
    -   [x] ROADMAP.md (future plans)
    -   [x] TODO.md (task tracking)
-   [x] Include root project configuration files:
    -   [x] .editorconfig
    -   [x] .gitignore
    -   [x] rustfmt.toml
    -   [x] clippy.toml
-   [x] Validate that generated projects pass:
    -   [x] `cargo check`
    -   [x] `cargo fmt`
    -   [x] `cargo clippy`

⸻

## Testing and Validation

**Goal:** Ensure all generated projects are reliable and production-grade.

-   [x] Write unit tests for CLI parsing and validation
-   [x] Write integration tests for end-to-end project generation
-   [x] Create test fixtures for validation scenarios
-   [x] Implement mock testing for interactive components
-   [x] Set up coverage reporting
-   [x] Make tests compatible with Miri for memory safety validation
-   [ ] Improve testing of interactive components:
    -   [ ] Add automated tests for dialoguer components
    -   [ ] Create more robust mocks for user input
    -   [ ] Test edge cases in user interaction
-   [ ] Scaffold mock project directories during tests
-   [ ] Validate that generated projects build successfully
-   [x] Validate that formatting, linting, and testing pass:
    -   [x] `cargo fmt`
    -   [x] `cargo clippy`
    -   [x] `cargo test`
    -   [x] `cargo nextest`
-   [ ] Verify optional tooling is correctly installed
-   [ ] Use `#[cfg(test)]` with `#[cfg(feature = "with-test-utils")]` for mocks
-   [ ] Create integration tests for:
    -   [ ] Wizard flow
    -   [ ] Manual CLI argument input
    -   [ ] TUI stub (can be no-op for now)
-   [ ] Mock `inquire` usage with `mockall` in `ui::prompts.rs`

⸻

## CI / Publish

**Goal:** Maintain high build quality and automate release processes.

-   [x] Doctor command (project/environment diagnostics)
-   [x] Optimize CI with specialized Rust caching (Leafwing-Studios/cargo-cache)
-   [x] Update deprecated toolchain actions to modern alternatives (crusty-pie/toolchain)
-   [ ] Set up GitHub Actions workflows:
    -   [ ] `cargo fmt` formatting check
    -   [ ] `cargo clippy` linting with denied warnings
    -   [ ] `cargo test` on stable, beta, nightly
-   [ ] Perform `cargo publish --dry-run` validation in CI
-   [ ] Integrate `cargo-release` or `cargo-smart-release` pipelines
-   [ ] Maintain Minimum Supported Rust Version (MSRV) tracking
-   [ ] Scaffold CI configuration templates into projects automatically

⸻

## Feature Flags & Modularization

**Goal:** Enhance flexibility and reduce dependencies through feature flags.

-   [ ] Add `[features]` block to root `Cargo.toml` with:
    -   [ ] `wizard` (default)
    -   [ ] `manual`
    -   [ ] `tui`
    -   [ ] `with-ui`
    -   [ ] `with-color-eyre` and `with-eyre`
    -   [ ] `with-time`
    -   [ ] `with-test-utils`
    -   [x] `with-benchmarks`
-   [ ] Gate dependencies using `optional = true` and `cfg(feature = "...")` in `Cargo.toml` and source code
-   [ ] Refactor crates to use features correctly in both `quickstart-lib` and `quickstart-cli`

⸻

## Macro Improvements

**Goal:** Simplify common patterns with well-designed macros.

-   [ ] Create macro module: `macros.rs`
-   [ ] Add:
    -   [ ] `cli_try!` → simplify `.map_err` into variant-wrapped errors
    -   [ ] `prompt_text!` → wrap `inquire::Text::new(...).with_help_message(...)`
    -   [ ] `wrap_suggest!` → unify `.wrap_err(...).suggest(...)` error handling

⸻

## Refactoring

**Goal:** Improve code organization and maintainability.

-   [ ] Move each mode into `src/mode/{manual, wizard, tui}.rs`
-   [ ] Move feature-gated UI helpers into `ui/` submodule
-   [ ] Clean up `main.rs` into just entrypoint logic

⸻

## Optional / Future Features

**Goal:** Explore additional functionality for future releases.

-   [ ] Add telemetry support (opt-in, anonymous)
-   [ ] Remote templates (HTTP fetch support)
-   [ ] Plugin architecture (runtime or compile-time)
-   [ ] TUI enhancements (keyboard navigation, preview)

⸻

Let's make `cargo-quickstart` the fastest, cleanest, and most powerful way to start a modern Rust project! 🐹
