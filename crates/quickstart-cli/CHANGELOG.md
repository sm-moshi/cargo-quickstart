# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.1 (2025-05-05)

### Added


### Changed


### Fixed

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 4 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`1e2e19b`](https://github.com/sm-moshi/cargo-quickstart/commit/1e2e19b9a90f8a9cba91a0a725b98c0e9dcf9c54))
    - Adjusting changelogs prior to release of quickstart-lib v0.1.2, cargo-quickstart v0.1.1 ([`49e30c6`](https://github.com/sm-moshi/cargo-quickstart/commit/49e30c6400b7f60fb38498d0b57527f81892cf33))
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

## 0.1.2 (2025-05-06)

### Added

- Added Miri compatibility for test suite
  - Modified filesystem operations with `cfg!(miri)` conditionals
  - Fixed error handling to work properly with Miri's strict memory safety checks
- Fixed error handling to work properly with Miri's strict memory safety checks

### Changed

- Improved CLI performance by optimizing underlying template engine operations
- Enhanced benchmark infrastructure using criterion and pprof

## 0.1.1 (2025-05-05)

