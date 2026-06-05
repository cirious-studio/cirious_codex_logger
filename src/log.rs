use crate::Level;

#[doc(hidden)]
/// Basic routing function (will be connected to dispatchers/formatters in the future)
pub fn _log(level: Level, msg: std::fmt::Arguments) {
  // Initial mock for stdout
  println!("[{:?}] {}", level, msg);
}

/// Logs a message at the error level.
///
/// Use this macro to log critical failures that prevent a component or the
/// entire application from continuing normal execution.
#[macro_export]
macro_rules! error {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Error, format_args!($($arg)+)))
}

/// Logs a message at the warn level.
///
/// Use this macro to log hazardous situations or anomalies that are not
/// fatal but should be investigated by the operations team.
#[macro_export]
macro_rules! warn {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Warn, format_args!($($arg)+)))
}

/// Logs a message at the info level.
///
/// Use this macro to record standard, high-level operational events like
/// startups, shutdowns, or significant milestones in your logic.
#[macro_export]
macro_rules! info {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Info, format_args!($($arg)+)))
}

/// Logs a message at the debug level.
///
/// Use this macro to log detailed diagnostic information useful for
/// developers attempting to understand internal program state or logic flow.
#[macro_export]
macro_rules! debug {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Debug, format_args!($($arg)+)))
}

/// Logs a message at the trace level.
///
/// Use this macro for extremely verbose logging, such as printing variables
/// inside tight loops, entering/exiting functions, or very granular details.
#[macro_export]
macro_rules! trace {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Trace, format_args!($($arg)+)))
}
