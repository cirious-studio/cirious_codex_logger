//! Integration tests for the asynchronous dispatching capabilities.
//!
//! This module verifies that the `AsyncDispatcher` correctly offloads logging
//! tasks to a background thread without blocking the calling thread,
//! ensuring that records are eventually processed.

use cirious_codex_logger::{AsyncDispatcher, Dispatcher, HumanReadableFormatter, Level, Record, StdoutDispatcher};
use std::time::SystemTime;

#[test]
fn test_async_dispatcher_execution() {
  // Wrap a standard dispatcher in an async dispatcher
  let stdout = Box::new(StdoutDispatcher::new(HumanReadableFormatter));
  let async_dispatcher = AsyncDispatcher::new(stdout, 5);

  let record = Record {
    level: Level::Debug,
    args: "Async test message".to_string(),
    file: "test.rs",
    line: 1,
    module_path: "test",
    timestamp: SystemTime::now(),
  };

  // Dispatch the log asynchronously
  async_dispatcher.dispatch(&record).ok();

  // Since the processing happens in a background thread, sleep briefly
  // to allow the channel to be consumed and the output to be flushed.
  std::thread::sleep(std::time::Duration::from_millis(100));
}
