# Changelog – cargo-quickstart

All notable changes to this project will be documented here.

⸻

## [Unreleased]

### Added

### Changed

### Fixed

⸻

## [v0.1.2]

### Added

- Added Miri compatibility for all tests
  - Modified filesystem operations with `if cfg!(miri)` conditionals
  - Fixed error handling in test helpers
  - Adjusted time-based operations in template handling
- Established benchmarking infrastructure for performance profiling
  - Added criterion-based benchmarks with pprof integration
  - Created `just perf-bench` and `just perf-cmd` commands for specific performance testing
  - Added flamegraph generation and visualization support
  - Identified template rendering as main performance bottleneck

### Changed

- Improved CLI performance by optimizing underlying template engine operations
- Enhanced just command structure with dedicated performance testing commands

### Fixed

⸻

## [v0.1.1]

### Added

- Exact version specifications between workspace crates to ensure consistent builds on crates.io

### Changed

- CI workflows now use `Leafwing-Studios/cargo-cache` for more efficient Rust-specific caching
- Updated from deprecated `actions-rs/toolchain` to `crusty-pie/toolchain`
- Replaced `lazy_static` dependency with standard library's `std::sync::LazyLock`

### Fixed

- Path validation now properly checks if parent directories exist before attempting to create project directories
- Added clear error messages for invalid paths to improve user experience

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
-   Colourful, user-friendly CLI output (console)
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

⸻
