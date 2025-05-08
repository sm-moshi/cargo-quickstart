# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

⸻

## v0.2.0 (2025-05-08)

### Added
- QuickstartConfig is now the canonical, extensible configuration struct for all UX modes (CLI, TUI, manual)
- Unified configuration logic, tests, and documentation on QuickstartConfig

### Changed
- All code, tests, and documentation now use QuickstartConfig
- ProjectConfig and all migration code fully removed

### Migration Notes
- If you were using ProjectConfig directly, migrate to QuickstartConfig. See documentation for field mapping and extensibility options.

⸻

## v0.1.4 (2025-05-06)

### Other

 - <csr-id-73c32e61d9bcce071f1e501b911d8a2895fe5fff/> removed protobuf 2.28.0 dependency

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #6 from sm-moshi/hotfix/GHSA-735f-pc8j-v9w8 ([`4bb14cd`](https://github.com/sm-moshi/cargo-quickstart/commit/4bb14cd31b277453d4899eedbee974c277ad4f17))
    - Removed protobuf 2.28.0 dependency ([`73c32e6`](https://github.com/sm-moshi/cargo-quickstart/commit/73c32e61d9bcce071f1e501b911d8a2895fe5fff))
    - Merge branch 'release/0.1.2' into develop ([`6e27b10`](https://github.com/sm-moshi/cargo-quickstart/commit/6e27b10b308dbe4d465009569629a1efe41dc6c7))
    - Quickstart-lib-v0.1.3 ([`cbd920f`](https://github.com/sm-moshi/cargo-quickstart/commit/cbd920f438be728c66b3cdbe8cae1e3c29484c5e))
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`9a20893`](https://github.com/sm-moshi/cargo-quickstart/commit/9a208930b1f8bf658802ad23c6b14a10e258effd))
    - Merge branch 'release/0.1.2' into develop ([`1d1e7de`](https://github.com/sm-moshi/cargo-quickstart/commit/1d1e7de3e6534a997b283f418a42c6a65e294f9b))
</details>

⸻

## v0.1.3 (2025-05-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Quickstart-lib-v0.1.3 ([`3334b3e`](https://github.com/sm-moshi/cargo-quickstart/commit/3334b3eeb3fa238a5d622e75b8ecf852d5403b76))
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`07cc5ec`](https://github.com/sm-moshi/cargo-quickstart/commit/07cc5ec2de6cd7d9802b00902fff26edaadc7b04))
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`1e2e19b`](https://github.com/sm-moshi/cargo-quickstart/commit/1e2e19b9a90f8a9cba91a0a725b98c0e9dcf9c54))
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`49e30c6`](https://github.com/sm-moshi/cargo-quickstart/commit/49e30c6400b7f60fb38498d0b57527f81892cf33))
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`9be2e2b`](https://github.com/sm-moshi/cargo-quickstart/commit/9be2e2b84fbe0645105f563215a8e76639868736))
    - Merge pull request #3 from sm-moshi/develop ([`31d692d`](https://github.com/sm-moshi/cargo-quickstart/commit/31d692d99a1cf42a2fc6f0394aa12b9c339315db))
    - Update crates/quickstart-lib/CHANGELOG.md ([`a70e565`](https://github.com/sm-moshi/cargo-quickstart/commit/a70e565ce5cbd8f676ddaee8be6270175e30a343))
</details>

⸻

## v0.1.2 (2025-05-06)

### Added

### Changed

### Fixed

## v0.1.1 (2025-05-05)

### Documentation

<csr-id-2e67d36371c90fabe1cc5fb2625d958a93347db9/>

 - <csr-id-1868c3db08e351db2940fadc77b829ae41ebe9a6/> update project documentation and memory bank
   This commit updates documentation to reflect the completed CLI implementation:

   - Enhance documentation files
   - Mark completed items in ROADMAP.md and TODO.md
- Add comprehensive CHANGELOG.md entries for implemented features
- Document test coverage details and approach
- Add new sections for interactive UI and testing improvements
- Update memory bank files with current project status
     - Document CLI framework completion in activeContext.md
     - Update progress.md with component completion status
     - Record testing approach and infrastructure details
- Update progress.md with component completion status
- Record testing approach and infrastructure details
- Mark completed items in ROADMAP.md and TODO.md
- Add comprehensive CHANGELOG.md entries for implemented features
- Document test coverage details and approach
- Add new sections for interactive UI and testing improvements

### New Features

<csr-id-ceaf9105d688626479b9defea548860e20b137cd/>
<csr-id-83197cce409fdd189ef3b412760ba3cabcfaf11d/>

 - <csr-id-e5b2b9bbfea532e9f53e91294d74371df239309c/> implement CLI scaffolding with robust test coverage
   This commit implements the complete CLI functionality including:

   - Full argument parsing with clap for new/init commands
 - Interactive prompts with dialoguer for user input
 - Project generation with proper error handling
 - Comprehensive test suite with:
       - Unit tests for internal functions
       - Integration tests with assert_cmd
       - Test fixtures for validation and mocking
       - Coverage reporting (74% line coverage)
 - Integration tests with assert_cmd
 - Test fixtures for validation and mocking
 - Coverage reporting (74% line coverage)
 - Full argument parsing with clap for new/init commands
 - Interactive prompts with dialoguer for user input
 - Project generation with proper error handling
 - Comprehensive test suite with:
       - Unit tests for internal functions
       - Integration tests with assert_cmd
       - Test fixtures for validation and mocking
       - Coverage reporting (74% line coverage)
 - Integration tests with assert_cmd
 - Test fixtures for validation and mocking
 - Coverage reporting (74% line coverage)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 5 calendar days.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release quickstart-lib v0.1.1 ([`ec24ba5`](https://github.com/sm-moshi/cargo-quickstart/commit/ec24ba55ff381af38a5967ac0ef56549fad8abe6))
    - Quickstart v0.1.1 CHANGELOG.md ([`8eb8066`](https://github.com/sm-moshi/cargo-quickstart/commit/8eb80663c3487d76920318064eb4ca63b671765c))
    - Merge pull request #2 from sm-moshi/develop ([`c116b81`](https://github.com/sm-moshi/cargo-quickstart/commit/c116b81f805fbfc558d33cb358868bc419906bef))
    - ~v0.1.1 ([`236bc17`](https://github.com/sm-moshi/cargo-quickstart/commit/236bc172bd592c9258b720e1ea9139cb4900c284))
    - Preparing v0.1.0 ([`d640d9f`](https://github.com/sm-moshi/cargo-quickstart/commit/d640d9fe5647aca15e28c45bfc75130bdf3b06be))
    - Meow ([`f3b283c`](https://github.com/sm-moshi/cargo-quickstart/commit/f3b283ca4b0e67f9c3a5e707d56a05cb70f0df3c))
    - Merge branch 'main' into develop ([`999b399`](https://github.com/sm-moshi/cargo-quickstart/commit/999b399048c5a8ca885d7627535299557c83f83b))
    - Implement CLI scaffolding with robust test coverage ([`e5b2b9b`](https://github.com/sm-moshi/cargo-quickstart/commit/e5b2b9bbfea532e9f53e91294d74371df239309c))
    - Update project documentation and memory bank ([`1868c3d`](https://github.com/sm-moshi/cargo-quickstart/commit/1868c3db08e351db2940fadc77b829ae41ebe9a6))
    - Implement CLI scaffolding with robust test coverage ([`ceaf910`](https://github.com/sm-moshi/cargo-quickstart/commit/ceaf9105d688626479b9defea548860e20b137cd))
    - INIT! ([`89bb640`](https://github.com/sm-moshi/cargo-quickstart/commit/89bb640aa132cd57f1fb4c4c40308f0b9473e4ff))
    - Merge branch 'release/v0.0.1' into develop ([`b2ea7df`](https://github.com/sm-moshi/cargo-quickstart/commit/b2ea7dff4daf97a944302e2af9c4bea166befd54))
    - Update project documentation and memory bank ([`2e67d36`](https://github.com/sm-moshi/cargo-quickstart/commit/2e67d36371c90fabe1cc5fb2625d958a93347db9))
    - Implement CLI scaffolding with robust test coverage ([`83197cc`](https://github.com/sm-moshi/cargo-quickstart/commit/83197cce409fdd189ef3b412760ba3cabcfaf11d))
    - INIT! ([`6039553`](https://github.com/sm-moshi/cargo-quickstart/commit/603955322f238fddba117ab02aa14466dfe707aa))
</details>

<csr-unknown>
The documentation now accurately reflects the project's ~74% test coverageand completed CLI framework with interactive components. update project documentation and memory bankThis commit updates documentation to reflect the completed CLI implementation:Enhance documentation filesThe documentation now accurately reflects the project's ~74% test coverageand completed CLI framework with interactive components.The implementation includes proper error propagation, separation of concerns between UI and logic, and follows idiomatic Rust patterns.All tests pass with appropriate use of mocking for code that requires user interaction. implement CLI scaffolding with robust test coverageThis commit implements the complete CLI functionality including:The implementation includes proper error propagation, separation of concerns between UI and logic, and follows idiomatic Rust patterns.All tests pass with appropriate use of mocking for code that requires user interaction. implement CLI scaffolding with robust test coverageThis commit implements the complete CLI functionality including:The implementation includes proper error propagation, separation of concerns between UI and logic, and follows idiomatic Rust patterns.All tests pass with appropriate use of mocking for code that requires user interaction.<csr-unknown/>

⸻
