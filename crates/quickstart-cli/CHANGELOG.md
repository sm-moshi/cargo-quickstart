# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

⸻

## [Unreleased] – 2025-05-09

### Added
- Scaffolded mode modules: wizard, manual, tui (stubs)
- Created config.rs and errors.rs stubs in quickstart-lib
- Updated project structure for multi-mode UX and future TUI integration
- Global `--dry-run` flag to CLI, now respected by wizard and TUI modes.
- TUI entrypoint (`run_tui`) now accepts and respects the `dry_run` flag.

### Changed
- Refactored CLI and library structure to support new UX model
- Updated documentation and cross-references for v0.2.x milestone
- Refactored wizard mode to use a `CommonArgs` trait for argument abstraction.
- Improved error handling in wizard mode: added `InquireError` and `LibraryError` variants.
- Updated all calls to `wizard::run` and `run_tui` to propagate the `dry_run` flag.

### Fixed
- Fixed all Clippy lints (field-reassign-with-default, uninlined-format-args).
- Removed unused imports and cleaned up mode modules.

⸻

## v0.2.0 (2025-05-08)

### Added

- QuickstartConfig is now the canonical, extensible configuration struct for all UX modes (CLI, TUI, manual)
- Unified configuration logic, tests, and documentation on QuickstartConfig

### Changed

- All code, tests, and documentation now use QuickstartConfig
- ProjectConfig and all migration code fully removed

### Migration Notes

- No user action required unless using the library API directly; see quickstart-lib/CHANGELOG.md for migration details.
- All crate versions (CLI and lib) are now synchronised to 0.2.0 after rebase/version correction. Workspace and all crates are version-aligned.

⸻

## 0.1.4 (2025-05-06)

### Other

- <csr-id-73c32e61d9bcce071f1e501b911d8a2895fe5fff/> removed protobuf 2.28.0 dependency

### Commit Statistics

<csr-read-only-do-not-edit/>

- 29 commits contributed to the release over the course of 6 calendar days.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized**
  - (bump): clap_complete 4.5.49 -> 4.5.50 ([`9cda788`](https://github.com/sm-moshi/cargo-quickstart/commit/9cda7888f7fe0ce3e19fc7d0e3e69f7bd51e7244))
  - Merge pull request #6 from sm-moshi/hotfix/GHSA-735f-pc8j-v9w8 ([`4bb14cd`](https://github.com/sm-moshi/cargo-quickstart/commit/4bb14cd31b277453d4899eedbee974c277ad4f17))
  - Removed protobuf 2.28.0 dependency ([`73c32e6`](https://github.com/sm-moshi/cargo-quickstart/commit/73c32e61d9bcce071f1e501b911d8a2895fe5fff))
  - Merge branch 'release/0.1.2' into develop ([`6e27b10`](https://github.com/sm-moshi/cargo-quickstart/commit/6e27b10b308dbe4d465009569629a1efe41dc6c7))
  - Quickstart-cli-v0.1.2 ([`8950d77`](https://github.com/sm-moshi/cargo-quickstart/commit/8950d77e53a4471e32ed4954be1e20d534bca2a1))
  - Quickstart-lib-v0.1.3 ([`cbd920f`](https://github.com/sm-moshi/cargo-quickstart/commit/cbd920f438be728c66b3cdbe8cae1e3c29484c5e))
  - Merge branch 'release/0.1.2' into develop ([`1d1e7de`](https://github.com/sm-moshi/cargo-quickstart/commit/1d1e7de3e6534a997b283f418a42c6a65e294f9b))
  - Quickstart-cli-v0.1.2 ([`c231083`](https://github.com/sm-moshi/cargo-quickstart/commit/c231083d2efccc842db6a0d24c16065786928c58))
  - Quickstart-lib-v0.1.3 ([`3334b3e`](https://github.com/sm-moshi/cargo-quickstart/commit/3334b3eeb3fa238a5d622e75b8ecf852d5403b76))
  - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`07cc5ec`](https://github.com/sm-moshi/cargo-quickstart/commit/07cc5ec2de6cd7d9802b00902fff26edaadc7b04))
  - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`1e2e19b`](https://github.com/sm-moshi/cargo-quickstart/commit/1e2e19b9a90f8a9cba91a0a725b98c0e9dcf9c54))
  - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`49e30c6`](https://github.com/sm-moshi/cargo-quickstart/commit/49e30c6400b7f60fb38498d0b57527f81892cf33))
  - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`9be2e2b`](https://github.com/sm-moshi/cargo-quickstart/commit/9be2e2b84fbe0645105f563215a8e76639868736))
  - Merge pull request #3 from sm-moshi/develop ([`31d692d`](https://github.com/sm-moshi/cargo-quickstart/commit/31d692d99a1cf42a2fc6f0394aa12b9c339315db))
  - Adjusting changelogs prior to release of cargo-quickstart v0.1.1 ([`d3700b5`](https://github.com/sm-moshi/cargo-quickstart/commit/d3700b534b2af392970037c5efb3b32a86001703))
  - Adjusting changelogs prior to release of cargo-quickstart v0.1.1 ([`6986bf6`](https://github.com/sm-moshi/cargo-quickstart/commit/6986bf6a69824e061b5b758930cdccbdb9ee0224))
  - Adjusting changelogs prior to release of cargo-quickstart v0.1.1 ([`951cce5`](https://github.com/sm-moshi/cargo-quickstart/commit/951cce549f7d495b85d7e31f43cb659bc7874e2a))
  - Adjusting changelogs prior to release of cargo-quickstart v0.1.1 ([`f5a171d`](https://github.com/sm-moshi/cargo-quickstart/commit/f5a171d237d68869f979431dd429d6c09826b5d6))
  - Adjusting changelogs prior to release of cargo-quickstart v0.1.1 ([`70ce637`](https://github.com/sm-moshi/cargo-quickstart/commit/70ce63710152954b3cc71ef64c4f055b797f3bd0))
  - Release quickstart-lib v0.1.1 ([`ec24ba5`](https://github.com/sm-moshi/cargo-quickstart/commit/ec24ba55ff381af38a5967ac0ef56549fad8abe6))
  - Quickstart v0.1.1 CHANGELOG.md ([`8eb8066`](https://github.com/sm-moshi/cargo-quickstart/commit/8eb80663c3487d76920318064eb4ca63b671765c))
  - Release v0.1.1 ([`a860321`](https://github.com/sm-moshi/cargo-quickstart/commit/a860321fe0dd17fbf9ac60e6a726ec78b4fda380))
  - Merge pull request #2 from sm-moshi/develop ([`c116b81`](https://github.com/sm-moshi/cargo-quickstart/commit/c116b81f805fbfc558d33cb358868bc419906bef))
  - ~v0.1.1 ([`236bc17`](https://github.com/sm-moshi/cargo-quickstart/commit/236bc172bd592c9258b720e1ea9139cb4900c284))
  - ~ ([`8351aaf`](https://github.com/sm-moshi/cargo-quickstart/commit/8351aaf214370f7fb7b96c2984c437ce2cc85340))
  - Preparing v0.1.0 ([`d640d9f`](https://github.com/sm-moshi/cargo-quickstart/commit/d640d9fe5647aca15e28c45bfc75130bdf3b06be))
  - Meow ([`f3b283c`](https://github.com/sm-moshi/cargo-quickstart/commit/f3b283ca4b0e67f9c3a5e707d56a05cb70f0df3c))
  - Merge branch 'main' into develop ([`999b399`](https://github.com/sm-moshi/cargo-quickstart/commit/999b399048c5a8ca885d7627535299557c83f83b))
  - Sorry. quickstart-cli was missing. ([`25f5c34`](https://github.com/sm-moshi/cargo-quickstart/commit/25f5c34d2bb2260693a856dc953c982406ee2a37))

</details>

⸻

## 0.1.2 (2025-05-06)

### Added

- Added Miri compatibility for test suite
  - Modified filesystem operations with `cfg!(miri)` conditionals
  - Fixed error handling to work properly with Miri's strict memory safety checks
- Fixed error handling to work properly with Miri's strict memory safety checks

### Changed

- Improved CLI performance by optimizing underlying template engine operations
- Enhanced benchmark infrastructure using criterion and pprof

<csr-unknown>
Fixed error handling to work properly with Miri's strict memory safety checks<csr-unknown/>

## v0.1.1 (2025-05-05)

### Added

### Changed

### Fixed

## 0.1.1 (2025-05-05)

## [Unreleased]

### Added

- Scaffolded mode modules: wizard, manual, tui (stubs)
- Created config.rs and errors.rs stubs in quickstart-lib
- Updated project structure for multi-mode UX and future TUI integration
- Global `--dry-run` flag to CLI, now respected by wizard and TUI modes.
- TUI entrypoint (`run_tui`) now accepts and respects the `dry_run` flag.

### Changed

- Refactored CLI and library structure to support new UX model
- Updated documentation and cross-references for v0.2.x milestone
- Refactored wizard mode to use a `CommonArgs` trait for argument abstraction.
- Improved error handling in wizard mode: added `InquireError` and `LibraryError` variants.
- Updated all calls to `wizard::run` and `run_tui` to propagate the `dry_run` flag.

### Fixed

- N/A
- Fixed all Clippy lints (field-reassign-with-default, uninlined-format-args).
- Removed unused imports and cleaned up mode modules.

## v0.2.0 (2025-05-08)

### Added

- QuickstartConfig is now the canonical, extensible configuration struct for all UX modes (CLI, TUI, manual)
- Unified configuration logic, tests, and documentation on QuickstartConfig

### Changed

- All code, tests, and documentation now use QuickstartConfig
- ProjectConfig and all migration code fully removed

### Migration Notes

- No user action required unless using the library API directly; see quickstart-lib/CHANGELOG.md for migration details.
