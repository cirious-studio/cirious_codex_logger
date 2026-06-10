use crate::Level;

/// Represents a single log event.
///
/// A `Record` contains all the necessary metadata and arguments required to
/// output a log message. It is passed from the logging macros to the active
/// dispatchers via a formatter.
#[derive(Clone, Debug)]
pub struct Record {
  /// The verbosity level of this record.
  pub level: Level,
  /// The formatted message arguments provided via the logging macros.
  pub args: String,
  /// The file name of the location where the log macro was invoked.
  pub file: &'static str,
  /// The line number of the location where the log macro was invoked.
  pub line: u32,
  /// The module path of the location where the log macro was invoked.
  pub module_path: &'static str,
  /// The timestamp of the log record.
  pub timestamp: std::time::SystemTime,
}

/// A trait for structuring and serializing log records.
///
/// Formatters take a `Record` and convert it into a `String` representation
/// suitable for the target dispatcher (e.g., plain text, JSON, etc.).
pub trait Formatter {
  /// Formats the given log record into a string.
  fn format(&self, record: &Record) -> String;
}

/// A standard, human-readable formatter.
///
/// This formatter outputs records in a simple plaintext format, making it
/// ideal for terminal output or basic text log files.
///
/// Format: `[Level] Message`
pub struct HumanReadableFormatter;

impl Formatter for HumanReadableFormatter {
  fn format(&self, record: &Record) -> String {
    format!("[{:?}] {}", record.level, record.args)
  }
}

/// A structured JSON formatter.
///
/// This formatter outputs log records as single-line JSON objects, which is
/// highly recommended for machine ingestion (e.g., Elasticsearch, Datadog).
#[derive(Debug)]
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
  fn format(&self, record: &Record) -> String {
    // Simple manual setup to avoid dependencies in the initial setup
    // In an advanced scenario, you can integrate `serde` or `serde_json`
    format!(r#"{{"level":"{:?}","message":"{}"}}"#, record.level, record.args)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Level;

  #[test]
  fn test_human_readable_formatter() {
    let args = format_args!("System initialized");
    let record = Record {
      level: Level::Debug,
      args: args.to_string(),
      file: "test",
      line: 1,
      module_path: "test",
      timestamp: std::time::SystemTime::now(),
    };

    let formatter = HumanReadableFormatter;
    let result = formatter.format(&record);

    assert_eq!(result, "[Debug] System initialized");
  }
}
