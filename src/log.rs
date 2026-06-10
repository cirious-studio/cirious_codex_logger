use std::{
  collections::BTreeMap,
  sync::{OnceLock, RwLock},
};

use crate::{Dispatcher, Level};

use std::sync::atomic::AtomicU8;

/// The global log level filter. It is initialized to `Level::Info` by default.
pub static GLOBAL_FILTER: AtomicU8 = AtomicU8::new(Level::Info as u8);

/// A map of module filters.
pub static MODULE_FILTERS: RwLock<BTreeMap<String, Level>> = RwLock::new(BTreeMap::new());

/// Set a filter for a specific module.
pub fn set_module_filter(module: &str, level: Level) {
  let mut filters = MODULE_FILTERS.write().unwrap();
  filters.insert(module.to_string(), level);
}

/// The global logger.
static GLOBAL_LOGGER: OnceLock<Box<dyn Dispatcher + Send + Sync>> = OnceLock::new();

/// Initialize the global logger with the given dispatcher.
pub fn init(dispatcher: Box<dyn Dispatcher + Send + Sync>) -> Result<(), Box<dyn Dispatcher + Send + Sync>> {
  cirious_codex_term::init_term();
  GLOBAL_LOGGER.set(dispatcher)
}

/// Get a reference to the global logger.
pub fn get_logger() -> Option<&'static (dyn Dispatcher + Send + Sync)> {
  GLOBAL_LOGGER.get().map(|d| d.as_ref())
}

// A private helper macro to handle the dispatch logic
#[macro_export]
#[doc(hidden)]
macro_rules! __log_internal {
  ($level:expr, $($arg:tt)+) => {
    let current_level = $level as u8;
    let module = module_path!();

    let allowed = if let Ok(filters) = $crate::log::MODULE_FILTERS.read() {
      if let Some(mod_level) = filters.get(module) {
        current_level >= (*mod_level as u8)
      } else {
        current_level >= $crate::log::GLOBAL_FILTER.load(std::sync::atomic::Ordering::Relaxed)
      }
    } else {
      current_level >= $crate::log::GLOBAL_FILTER.load(std::sync::atomic::Ordering::Relaxed)
    };

    if allowed {
      if let Some(logger) = $crate::get_logger() {
        let record = $crate::format::Record {
          level: $level,
          args: format_args!($($arg)+),
          file: file!(),
          line: line!(),
          module_path: module_path!(),
          timestamp: std::time::SystemTime::now(),
        };
        logger.dispatch(&record);
      }
    }
  };
}

/// Logs a message at the error level.
///
/// Use this macro to log critical failures that prevent a component or the
/// entire application from continuing normal execution.
#[macro_export]
macro_rules! error { ($($arg:tt)+) => { $crate::__log_internal!($crate::level::Level::Error, $($arg)+) } }

/// Logs a message at the warn level.
///
/// Use this macro to log hazardous situations or anomalies that are not
/// fatal but should be investigated by the operations team.
#[macro_export]
macro_rules! warn  { ($($arg:tt)+) => { $crate::__log_internal!($crate::level::Level::Warn,  $($arg)+) } }

/// Logs a message at the info level.
///
/// Use this macro to record standard, high-level operational events like
/// startups, shutdowns, or significant milestones in your logic.
#[macro_export]
macro_rules! info  { ($($arg:tt)+) => { $crate::__log_internal!($crate::level::Level::Info,  $($arg)+) } }

/// Logs a message at the debug level.
///
/// Use this macro to log detailed diagnostic information useful for
/// developers attempting to understand internal program state or logic flow.
#[macro_export]
macro_rules! debug { ($($arg:tt)+) => { $crate::__log_internal!($crate::level::Level::Debug, $($arg)+) } }

/// Logs a message at the trace level.
///
/// Use this macro for extremely verbose logging, such as printing variables
/// inside tight loops, entering/exiting functions, or very granular details.
#[macro_export]
macro_rules! trace { ($($arg:tt)+) => { $crate::__log_internal!($crate::level::Level::Trace, $($arg)+) } }
