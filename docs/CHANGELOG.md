# Changelog – cargo-quickstart

All notable changes to this project will be documented here.

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

⸻

## [Unreleased]

### Added
-   Template system: robust, fully tested, and production-ready
-   Handlebars-based template engine and loader
-   Structured `templates/` directory at project root
-   Loader path resolution (searches upwards for nearest `templates/`)
-   `.hbs`-only filter for template files
-   Placement rule: files from `base/` are placed at project root
-   Comprehensive unit and integration test coverage for template system
-   Shell completions for Bash, Zsh, Fish, Powershell, and Elvish
-   Colourful, user-friendly CLI output (owo-colors)
-   CLI output polish and improved UX

### Changed
-   Project generation logic is now template-driven
-   CLI output is now consistent, colourised, and user-friendly

### Fixed
-   All template system tests and lints pass
-   All workspace lints and tests pass (only known cargo-udeps false positives in the library crate)

### Next Focus
-   Doctor command (project/environment diagnostics)
-   Template info/discovery expansion
-   Remote/custom template support
-   Interactive mode
-   Config file support

---
