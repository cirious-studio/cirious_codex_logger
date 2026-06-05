#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  Error = 1,
  Warn,
  Info,
  Debug,
  Trace,
}

// Basic routing function (will be connected to dispatchers/formatters in the future)
#[doc(hidden)]
pub fn _log(level: Level, msg: std::fmt::Arguments) {
  // Initial mock for stdout
  println!("[{:?}] {}", level, msg);
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Error, format_args!($($arg)+)))
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Warn, format_args!($($arg)+)))
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Info, format_args!($($arg)+)))
}

#[macro_export]
macro_rules! debug {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Debug, format_args!($($arg)+)))
}

#[macro_export]
macro_rules! trace {
  ($($arg:tt)+) => ($crate::_log($crate::Level::Trace, format_args!($($arg)+)))
}
