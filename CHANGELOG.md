# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-10

### Added
- **Global Logger Registration:** Implemented `init()` with `OnceLock` for thread-safe global access.
- **Metadata Enrichment:** Macros now automatically capture `timestamp`, `module_path`, `file`, and `line` information.
- **Log Filtering:** Added granular module-based and level-based filtering capabilities.
- **Async Dispatching:** Introduced `AsyncDispatcher` to offload I/O operations to a background thread.
- **File Rotation:** Implemented `RollingFileDispatcher` with size-based rotation and backup logic.
- **Formatting Improvements:** Enhanced `HumanReadableFormatter` and `JsonFormatter` API for better extensibility.
- **Integration Tests:** Added full test suite in `tests/` covering initialization, async flow, and file rotation.
- **Examples:** Created comprehensive usage examples in the `examples/` directory.

### Changed
- Refactored `Dispatcher` trait to allow more flexible and performant I/O operations.
- Updated crate documentation and `README.md` with architectural diagrams.

## [0.1.0] - 2026-04-15

### Added
- Core logging macros: `trace!`, `debug!`, `info!`, `warn!`, `error!`.
- Initial `Stdout` and `Stderr` dispatchers.
- Basic terminal styling via `cirious_codex_term`.
- Initial project structure and CI pipeline.
