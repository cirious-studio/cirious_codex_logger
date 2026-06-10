//! Integration tests for the file rotation functionality of the `RollingFileDispatcher`.
//!
//! This module verifies that the dispatcher correctly identifies when the
//! log file size exceeds the configured threshold and triggers a file rotation.

use cirious_codex_logger::{Dispatcher, HumanReadableFormatter, Level, Record, RollingFileDispatcher};
use std::fs;
use std::time::SystemTime;

#[test]
fn test_rolling_file_rotation() {
  let path = "test_rotation.log";
  // Ensure clean state before starting
  let _ = fs::remove_file(path);
  let _ = fs::remove_file("test_rotation.log.old");

  let formatter = HumanReadableFormatter;
  let max_size = 20; // Very small to force immediate rotation
  let dispatcher = RollingFileDispatcher::new(path, max_size, formatter).ok();

  let record = Record {
    level: Level::Info,
    args: "Trigger".to_string(), // Short string
    file: "test.rs",
    line: 1,
    module_path: "test",
    timestamp: SystemTime::now(),
  };

  if let Some(dispatcher) = dispatcher {
    // First write: file size is small, no rotation
    dispatcher.dispatch(&record).ok();
    // Second write: size will exceed 20, rotation SHOULD trigger
    dispatcher.dispatch(&record).ok();
  }

  // Give the file system a millisecond to update metadata
  std::thread::sleep(std::time::Duration::from_millis(10));

  assert!(
    fs::metadata("test_rotation.log.old").is_ok(),
    "The backup file (.log.old) should exist after rotation"
  );

  let _ = fs::remove_file(path);
  let _ = fs::remove_file("test_rotation.log.old");
}
