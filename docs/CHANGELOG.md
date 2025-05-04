# Changelog – cargo-quickstart

All notable changes to this project will be documented here.

⸻

## [Unreleased]

### Added

- *No unreleased changes yet*

⸻

## [v0.1.0]

### Added
-   VS Code configuration templates for all generated projects:
    -   `.vscode/settings.json` with best-practice Rust editor settings
    -   `.vscode/extensions.json` with recommended extensions for Rust development
    -   `.vscode/launch.json` with debug configurations for both binary and library projects
    -   `.vscode/tasks.json` with common cargo commands and problem matchers
-   Template system: robust, fully tested, and production-ready
    -   Handlebars-based template engine and loader
    -   Structured `templates/` directory at project root
    -   Loader path resolution (searches upwards for nearest `templates/`)
    -   `.hbs`-only filter for template files
    -   Placement rule: files from `base/` are placed at project root
    -   Comprehensive unit and integration test coverage
-   Shell completions for Bash, Zsh, Fish, Powershell, and Elvish
-   Colourful, user-friendly CLI output (owo-colors)
-   Doctor command implementation:
    -   Project health diagnostics with scoring
    -   Dependency checks (cargo-outdated, cargo-udeps)
    -   File structure validation
    -   Rust toolchain verification
    -   Template system checks
    -   Lint configuration validation
    -   98 passing tests with full coverage
    -   Performance optimized command execution
-   Added documentation stub templates (CHANGELOG.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md) to all generated projects for improved onboarding and compliance with best practices.

### Changed
-   Project generation logic is now template-driven
-   Template variables unified to consistently use `name` (replacing `project.name` and `crate_name`)
-   Dynamic year generation in CHANGELOG.md using `date.year` variable
-   CLI output is now consistent, colourised, and user-friendly
-   Test suite expanded to 98 passing tests
-   Doctor command performance optimized:
    -   Core checks complete in < 0.5s
    -   Full diagnostics complete in < 10s
    -   Memory usage optimized with no leaks

### Fixed
-   Template variables consistently use `name` and `project.is_binary`/`project.is_library`
-   All template system tests and lints pass
-   All workspace lints and tests pass
-   Doctor command memory leak resolved
-   Test isolation improved for doctor command
-   Mock command executors working correctly
-   Dependency checks properly handling all scenarios

⸻

## [v0.0.1]

### Added

-   Initial CLI structure using `clap`:
    -   Root command with proper version and description
    -   `new` and `init` subcommands with appropriate flags
    -   Command-line arguments parsing and validation
-   Interactive UI with `dialoguer`:
    -   Project name input with validation
    -   Project type selection (binary/library)
    -   Configuration confirmation
    -   Progress spinners for long-running operations
    -   Input validation with helpful error messages
-   Error handling with `color-eyre`:
    -   Graceful error reporting with context
    -   Command error extension traits
    -   User-friendly error messages with suggested actions
-   UI components with `owo-colors`:
    -   Colorful terminal output for improved readability
    -   Progress indicators with `indicatif`
    -   Consistent message formatting and styling
-   Test infrastructure:
    -   Unit tests for CLI parsing and command implementation
    -   Integration tests with `assert_cmd`
    -   Test fixtures for validation scenarios
    -   Mock testing patterns for interactive components
    -   Manual test scaffolding for interactive prompts
    -   Coverage reporting (74% overall line coverage)
-   Project Configuration:
    -   Core project config types and validation
    -   Project type enum (Binary/Library)
    -   License selection and validation

### Changed

-   N/A

### Deprecated

-   N/A

### Removed

-   N/A

### Fixed

-   N/A

### Security

-   N/A

---
