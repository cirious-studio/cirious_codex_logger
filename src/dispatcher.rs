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
      args,
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
      args,
    };

    let dispatcher = StderrDispatcher::new(JsonFormatter);

    // Ensures no panics occur when dispatching to stderr
    dispatcher.dispatch(&record);
  }
}
