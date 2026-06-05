//! # Cirious Codex Logger
//!
//! A highly optimized, foundational library for structured logging and event tracing
//! designed as the ultimate observability bedrock for the Cirious ecosystem.
//!
//! This crate provides efficient mechanisms for capturing, formatting, and dispatching
//! application events with rich metadata, prioritizing performance, structural integrity,
//! and seamless terminal integration.
//!
//! ## Quick Start
//!
//! Simply use the provided core macros to easily emit structured logs anywhere in your
//! application.
//!
//! ```rust
//! use cirious_codex_logger::{debug, error, info, trace, warn};
//!
//! // Emit informational events
//! info!("Application started successfully.");
//!
//! // Trace execution flow or debug variables
//! debug!("Debugging an internal variable: {}", 42);
//! trace!("Tracing execution flow.");
//!
//! // Log warnings and critical errors
//! warn!("Memory usage is high.");
//! error!("Failed to connect to the database.");
//! ```

#![warn(missing_docs)]

/// Routing and dispatching mechanisms for log records.
///
/// Includes implementations for standard out and standard error streams.
pub mod dispatcher;

/// Utilities for structuring and serializing log events.
///
/// Provides formatters for both human-readable terminal output and structured JSON.
pub mod format;

/// Definitions for logging verbosity levels.
///
/// Controls the severity and filtering of the emitted log records.
pub mod level;

/// The core logging macros and global logging entry points.
///
/// This module provides the essential `info!`, `error!`, `warn!`, `debug!`, and `trace!` macros.
pub mod log;

/// Terminal styling and colorization utilities.
///
/// Integrates with the Cirious ecosystem to provide visually distinct and rich terminal outputs.
pub mod style;

pub use dispatcher::{Dispatcher, StderrDispatcher, StdoutDispatcher};
pub use format::{Formatter, HumanReadableFormatter, JsonFormatter, Record};
pub use level::Level;
pub use log::*;
pub use style::{level_color, StyledTerminalFormatter};
