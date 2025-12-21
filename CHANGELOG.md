# Changelog

All notable changes to this project will be documented in this file.

We aim to follow Semantic Versioning. While <1.0, we try to keep minor releases backwards compatible when reasonable.

## [Unreleased] (target: 0.2.0-alpha.1)

### Added
- `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `SECURITY.md`.
- `CHANGELOG.md`.
- GitHub Actions CI workflow (fmt, clippy, test).
- `.gitignore` entry to ignore `cargo-mutants` artifacts (`/mutants.out*`).
- Snapshot testing with `insta` to stabilize metrics expectations.
- Cargo-mutants configuration to exclude an equivalent mutant.

### Changed
- Expanded and adjusted test fixtures to cover additional metrics paths (e.g., `pub fn`, block TODOs).

### Fixed
- Clippy warnings in analysis and output modules (no functional changes intended).
- Strengthened tests guided by `cargo-mutants` (all non-equivalent mutants now caught).


## [0.1.0] - 2025-11-25
- Initial release.
