/// Describes the verbosity level of a log message.
///
/// Levels are ordered from most critical to least critical (e.g., `Error` is
/// considered "less" than `Warn` in terms of verbosity, but it has a higher
/// priority). This allows for easy filtering (e.g., `level <= Level::Info`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  /// Designates very serious errors that may halt the application or
  /// indicate a severe failure.
  Error = 1,

  /// Designates hazardous situations that are not necessarily fatal,
  /// but should be investigated.
  Warn,

  /// Designates useful information about the normal operation of the system.
  Info,

  /// Designates lower priority information useful for debugging the
  /// internal state of the program.
  Debug,

  /// Designates very low priority, often extremely verbose, information
  /// used to trace execution flow.
  Trace,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_level_ordering() {
    assert!(Level::Error < Level::Warn);
    assert!(Level::Trace > Level::Info);
  }
}
