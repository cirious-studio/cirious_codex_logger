//! Integration tests for the Cirious Codex Logger initialization and dispatch workflow.
//!
//! This module ensures that the global logger can be successfully initialized
//! and that the logging macros interact correctly with the dispatcher at runtime.

use cirious_codex_logger::{info, init, HumanReadableFormatter, StdoutDispatcher};

#[test]
fn test_logger_initialization_and_dispatch() {
  // Set up a standard stdout dispatcher for testing
  let formatter = HumanReadableFormatter;
  let dispatcher = Box::new(StdoutDispatcher::new(formatter));

  // Initialize the global logger
  let result = init(dispatcher);

  // Assert that the logger was initialized successfully
  assert!(result.is_ok(), "The logger should have been initialized successfully");

  // Verify that the info! macro functions correctly post-initialization
  info!("Integration test running successfully");
}
