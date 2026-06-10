use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Mutex;
use std::thread;

use crate::format::{Formatter, Record};

/// A type alias for the result of a dispatch operation.
///
/// This type ensures that errors returned by dispatchers are thread-safe (`Send` + `Sync`),
/// allowing them to be safely propagated across threads in asynchronous contexts.
pub type DispatchResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// A trait representing a destination for log records.
///
/// Dispatchers are responsible for routing formatted log records to specific
/// targets such as `stdout`, `stderr`, files, or network sockets.
pub trait Dispatcher: Send + Sync + std::fmt::Debug {
  /// Dispatches a log record to the underlying destination.
  ///
  /// # Arguments
  /// * `record` - The [`Record`] containing the log data to be dispatched.
  ///
  /// # Returns
  /// A [`DispatchResult`] which is `Ok(())` on success, or an error containing
  /// a boxed error if the dispatch operation fails.
  ///
  /// # Errors
  /// Returns a boxed error if the dispatch operation fails.
  fn dispatch(&self, record: &Record) -> DispatchResult;
}

/// A dispatcher that writes log records to standard output (`stdout`).
#[derive(Debug)]
pub struct StdoutDispatcher<F: Formatter> {
  formatter: F,
}

impl<F: Formatter> StdoutDispatcher<F> {
  /// Creates a new `StdoutDispatcher` with the specified `formatter`.
  ///
  /// The dispatcher will use the provided `formatter` to transform every
  /// log [`Record`] into a string before outputting it to the standard output.
  ///
  /// # Arguments
  /// * `formatter` - An implementation of the [`Formatter`] trait that defines
  ///   the visual style and layout of the log output.
  ///
  /// # Example
  /// ```
  /// use cirious_codex_logger::{StdoutDispatcher, StyledTerminalFormatter};
  ///
  /// let formatter = StyledTerminalFormatter;
  /// let dispatcher = StdoutDispatcher::new(formatter);
  /// ```
  #[must_use]
  pub const fn new(formatter: F) -> Self {
    Self { formatter }
  }
}

impl<F: Formatter + Send + Sync + std::fmt::Debug> Dispatcher for StdoutDispatcher<F> {
  fn dispatch(&self, record: &Record) -> DispatchResult {
    println!("{}", self.formatter.format(record));
    Ok(())
  }
}

/// A dispatcher that writes log records to standard error (`stderr`).
#[derive(Debug)]
pub struct StderrDispatcher<F: Formatter> {
  formatter: F,
}

impl<F: Formatter> StderrDispatcher<F> {
  /// Creates a new `StderrDispatcher` with the specified `formatter`.
  ///
  /// The dispatcher uses the provided `formatter` to transform every
  /// log [`Record`] into a string, which is then written to the standard
  /// error (stderr) stream.
  ///
  /// # Arguments
  /// * `formatter` - An implementation of the [`Formatter`] trait used to
  ///   structure the output.
  ///
  /// # Example
  /// ```
  /// use cirious_codex_logger::{StderrDispatcher, JsonFormatter};
  ///
  /// let formatter = JsonFormatter;
  /// let dispatcher = StderrDispatcher::new(formatter);
  /// ```
  #[must_use]
  pub const fn new(formatter: F) -> Self {
    Self { formatter }
  }
}

impl<F: Formatter + Send + Sync + std::fmt::Debug> Dispatcher for StderrDispatcher<F> {
  fn dispatch(&self, record: &Record) -> DispatchResult {
    eprintln!("{}", self.formatter.format(record));
    Ok(())
  }
}

/// A dispatcher that offloads log processing to a background thread.
#[derive(Debug)]
pub struct AsyncDispatcher {
  sender: SyncSender<Record>,
}

impl AsyncDispatcher {
  /// Creates a new `AsyncDispatcher` that wraps the provided `dispatcher`.
  ///
  /// This constructor spawns a dedicated background thread that manages log
  /// dispatching. All log records sent to this dispatcher are queued in an
  /// internal bounded channel before being processed asynchronously.
  ///
  /// # Arguments
  /// * `dispatcher` - The underlying [`Dispatcher`] responsible for the actual
  ///   output (e.g., writing to a file or standard streams).
  /// * `buffer_size` - The maximum capacity of the internal channel. When the
  ///   buffer is full, logging threads will block until space becomes available
  ///   (backpressure).
  ///
  /// # Panics
  /// This method will panic if the operating system fails to spawn the
  /// background OS thread.
  ///
  /// # Example
  /// ```
  /// use cirious_codex_logger::{AsyncDispatcher, StdoutDispatcher, StyledTerminalFormatter};
  ///
  /// let stdout_dispatcher = Box::new(StdoutDispatcher::new(StyledTerminalFormatter));
  /// let async_dispatcher = AsyncDispatcher::new(stdout_dispatcher, 1024);
  /// ```
  #[must_use]
  pub fn new(dispatcher: Box<dyn Dispatcher + Send + Sync>, buffer_size: usize) -> Self {
    let (sender, receiver) = sync_channel::<Record>(buffer_size);

    thread::spawn(move || {
      while let Ok(record) = receiver.recv() {
        dispatcher.dispatch(&record).ok();
      }
    });

    Self { sender }
  }
}

impl Dispatcher for AsyncDispatcher {
  fn dispatch(&self, record: &Record) -> DispatchResult {
    let _ = self.sender.send(record.clone());
    Ok(())
  }
}

/// A dispatcher that writes to a file and rotates it when it exceeds `max_size`.
#[derive(Debug)]
pub struct RollingFileDispatcher<F: Formatter> {
  file_path: PathBuf,
  max_size: u64,
  inner: Mutex<File>,
  formatter: F,
}

impl<F: Formatter> RollingFileDispatcher<F> {
  /// Creates a new `RollingFileDispatcher` that writes logs to the specified file.
  ///
  /// When the file size reaches `max_size` bytes, the dispatcher automatically
  /// rotates the current log file by renaming it to `<filename>.log.old`
  /// and starting a fresh file.
  ///
  /// # Arguments
  /// * `path` - The file system path where the log file should be created/appended.
  /// * `max_size` - The threshold in bytes before the file rotation is triggered.
  /// * `formatter` - The [`Formatter`] used to structure the log output.
  ///
  /// # Errors
  /// Returns a `std::io::Error` if the file cannot be opened or created
  /// at the specified path.
  ///
  /// # Example
  /// ```
  /// use cirious_codex_logger::{RollingFileDispatcher, StyledTerminalFormatter};
  ///
  /// let formatter = StyledTerminalFormatter;
  /// let dispatcher = RollingFileDispatcher::new("app.log", 10 * 1024 * 1024, formatter)
  ///     .expect("Failed to initialize rolling file dispatcher");
  /// ```
  pub fn new<P: AsRef<Path>>(path: P, max_size: u64, formatter: F) -> Result<Self, std::io::Error> {
    let file = OpenOptions::new().append(true).create(true).open(&path)?;
    Ok(Self {
      file_path: path.as_ref().to_path_buf(),
      max_size,
      inner: Mutex::new(file),
      formatter,
    })
  }

  /// Rotates the current log file by renaming it to a backup path and creating
  /// a new, empty log file.
  ///
  /// This method performs the following steps:
  /// 1. Renames the current log file to `<original_filename>.log.old`,
  ///    overwriting any existing backup file.
  /// 2. Creates a new log file at the original path to resume logging.
  ///
  /// # Arguments
  /// * `file` - A mutable reference to the currently open log file handle.
  ///   The handle will be updated to point to the new file upon success.
  ///
  /// # Errors
  /// Returns a `std::io::Error` if:
  /// * The system fails to rename the current file.
  /// * The system fails to create the new log file.
  fn rotate(&self, file: &mut File) -> std::io::Result<()> {
    let backup_path = self.file_path.with_extension("log.old");
    std::fs::rename(&self.file_path, backup_path)?;
    *file = File::create(&self.file_path)?;
    Ok(())
  }
}

impl<F: Formatter + Send + Sync + std::fmt::Debug> Dispatcher for RollingFileDispatcher<F> {
  fn dispatch(&self, record: &Record) -> DispatchResult {
    let msg = self.formatter.format(record);
    let mut file = self.inner.lock().map_err(|_| "Failed to acquire lock")?;

    if let Ok(metadata) = file.metadata() {
      if metadata.len() + msg.len() as u64 > self.max_size {
        if let Err(e) = self.rotate(&mut file) {
          eprintln!("Rotation failed: {e}");
        }
      }
    }

    writeln!(file, "{msg}")?;
    file.flush()?;
    drop(file);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::format::JsonFormatter;
  use crate::Level;
  use std::time::SystemTime;

  /// Helper to generate a dummy record for testing purposes.
  fn create_test_record(level: Level, message: &str) -> Record {
    Record {
      level,
      args: message.to_string(),
      file: "test.rs",
      line: 42,
      module_path: "test_module",
      timestamp: SystemTime::now(),
    }
  }

  #[test]
  fn test_stdout_dispatcher_dispatch() {
    let record = create_test_record(Level::Info, "Testing stdout dispatcher");
    let dispatcher = StdoutDispatcher::new(JsonFormatter);

    // Ensure dispatching does not panic
    dispatcher.dispatch(&record).ok();
  }

  #[test]
  fn test_stderr_dispatcher_dispatch() {
    let record = create_test_record(Level::Error, "Testing stderr dispatcher");
    let dispatcher = StderrDispatcher::new(JsonFormatter);

    // Ensure dispatching does not panic
    dispatcher.dispatch(&record).ok();
  }

  #[test]
  fn test_async_dispatcher_dispatch() {
    let record = create_test_record(Level::Warn, "Testing async dispatcher");
    let stdout = Box::new(StdoutDispatcher::new(JsonFormatter));
    let async_dispatcher = AsyncDispatcher::new(stdout, 10);

    // Ensure async dispatching does not panic
    async_dispatcher.dispatch(&record).ok();
  }
}
