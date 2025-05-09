# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

⸻

## [Unreleased] – 2025-05-09

### Added
- Initial TUI mode implementation for cargo-quickstart (ratatui + crossterm).
- Public API: `run_tui(initial_project_name: Option<String>, dry_run: bool)`.
- Integration with CLI: TUI now respects the global `--dry-run` flag.

### Changed
- Improved state management and event loop for interactive project creation.
- Enhanced error propagation and user feedback in TUI mode.

### Fixed
- N/A

⸻
