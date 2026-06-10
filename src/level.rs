/// Describes the verbosity level of a log message.
///
/// Levels are ordered by severity. When filtering logs, you typically want to
/// log everything with a severity greater than or equal to a threshold.
///
/// # Hierarchy
/// The variants are assigned explicit discriminants to ensure that `Error`
/// represents the highest severity (lowest numeric value) and `Trace` the lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  /// Serious failures that may halt the application.
  Error = 1,
  /// Hazardous situations that warrant investigation.
  Warn = 2,
  /// Informational messages regarding normal operation.
  Info = 3,
  /// Low-priority information for internal debugging.
  Debug = 4,
  /// Verbose information for tracing execution flow.
  Trace = 5,
}

impl Default for Level {
  /// Returns the default log level (`Level::Info`).
  fn default() -> Self {
    Self::Info
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_level_ordering() {
    // Asserting that severity increases as the discriminant decreases
    assert!(Level::Error < Level::Warn);
    assert!(Level::Info > Level::Warn);
    assert!(Level::Trace > Level::Debug);
  }

  #[test]
  fn test_default_level() {
    assert_eq!(Level::default(), Level::Info);
  }
}
