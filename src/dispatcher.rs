use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

use crate::format::{Formatter, Record};

/// A trait representing a destination for log records.
///
/// Dispatchers are responsible for taking a formatted log record and outputting
/// it to a specific target (e.g., standard output, standard error, a file, etc.).
/// All dispatchers must be thread-safe (`Send + Sync`) to allow global logging.
pub trait Dispatcher: Send + Sync + std::fmt::Debug {
  /// Dispatches a log record to the underlying destination.
  ///
  /// This method is called by the logging macros whenever a new event is emitted.
  fn dispatch(&self, record: &Record);
}

/// A dispatcher that writes log records to standard output (`stdout`).
///
/// This dispatcher uses the provided `Formatter` to convert the `Record` into
/// a string before printing it via `println!`.
#[derive(Debug)]
pub struct StdoutDispatcher<F: Formatter> {
  formatter: F,
}

impl<F: Formatter> StdoutDispatcher<F> {
  /// Creates a new `StdoutDispatcher` with the given formatter.
  pub fn new(formatter: F) -> Self {
    Self { formatter }
  }
}

impl<F: Formatter + Send + Sync + std::fmt::Debug> Dispatcher for StdoutDispatcher<F> {
  fn dispatch(&self, record: &Record) {
    let output = self.formatter.format(record);
    println!("{}", output);
  }
}

/// A dispatcher that writes log records to standard error (`stderr`).
///
/// This dispatcher uses the provided `Formatter` to convert the `Record` into
/// a string before printing it via `eprintln!`.
#[derive(Debug)]
pub struct StderrDispatcher<F: Formatter> {
  formatter: F,
}

impl<F: Formatter + std::fmt::Debug> StderrDispatcher<F> {
  /// Creates a new `StderrDispatcher` with the given formatter.
  pub fn new(formatter: F) -> Self {
    Self { formatter }
  }
}

impl<F: Formatter + Send + Sync + std::fmt::Debug> Dispatcher for StderrDispatcher<F> {
  fn dispatch(&self, record: &Record) {
    let output = self.formatter.format(record);
    eprintln!("{}", output); // Outputs to Standard Error
  }
}

/// A dispatcher that offloads log record processing to a dedicated background thread.
///
/// The `AsyncDispatcher` acts as a wrapper around an underlying `Dispatcher`,
/// decoupling the log-generating threads from the I/O-intensive task of dispatching
/// log records.
///
/// # Mechanism
/// It utilizes a bounded channel (`std::sync::mpsc::sync_channel`) to queue
/// [`Record`] instances. A background thread continuously consumes these records
/// and forwards them to the wrapped dispatcher. This prevents the main execution
/// path from blocking during I/O operations (e.g., writing to a slow file system
/// or a terminal).
///
/// # Backpressure
/// Because the channel is bounded by the `buffer_size`, the `AsyncDispatcher`
/// inherently provides backpressure. If the logger falls behind the rate of incoming
/// log events, the main thread will block until space becomes available in the buffer,
/// preventing unbounded memory growth.
#[derive(Debug)]
pub struct AsyncDispatcher {
  sender: SyncSender<Record>,
}

impl AsyncDispatcher {
  /// Creates a new `AsyncDispatcher` that wraps the provided `dispatcher`.
  ///
  /// # Arguments
  /// * `dispatcher` - The destination to which records will eventually be sent.
  /// * `buffer_size` - The maximum number of log records to queue before the
  ///   logging threads are blocked (backpressure limit).
  ///
  /// # Panics
  /// This will spawn a new OS thread. If the operating system fails to create
  /// the thread, this method will panic.
  pub fn new(dispatcher: Box<dyn Dispatcher + Send + Sync>, buffer_size: usize) -> Self {
    let (sender, receiver) = sync_channel::<Record>(buffer_size);

    thread::spawn(move || {
      // The background thread stays alive as long as the sender exists.
      while let Ok(record) = receiver.recv() {
        dispatcher.dispatch(&record);
      }
    });

    Self { sender }
  }
}

impl Dispatcher for AsyncDispatcher {
  fn dispatch(&self, record: &Record) {
    self.sender.send(record.clone()).ok();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::format::JsonFormatter;
  use crate::Level;

  #[test]
  fn test_stdout_dispatcher_execution() {
    let args = format_args!("Testing stdout dispatcher");
    let record = Record {
      level: Level::Info,
      args: args.to_string(),
      file: "test",
      line: 1,
      module_path: "test",
      timestamp: std::time::SystemTime::now(),
    };

    // Constructing the dispatcher with a specific formatter
    let dispatcher = StdoutDispatcher::new(JsonFormatter);

    // This will actually print to the test console,
    // but the main goal is to ensure it doesn't panic.
    dispatcher.dispatch(&record);
  }

  #[test]
  fn test_stderr_dispatcher_execution() {
    let args = format_args!("Testing stderr dispatcher");
    let record = Record {
      level: Level::Error,
      args: args.to_string(),
      file: "test",
      line: 1,
      module_path: "test",
      timestamp: std::time::SystemTime::now(),
    };

    let dispatcher = StderrDispatcher::new(JsonFormatter);

    // Ensures no panics occur when dispatching to stderr
    dispatcher.dispatch(&record);
  }
}
