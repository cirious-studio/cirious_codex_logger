use crate::format::{Formatter, Record};

// 1. Base trait for any output destination
pub trait Dispatcher: Send + Sync {
  fn dispatch(&self, record: &Record);
}

// 2. Stdout implementation
pub struct StdoutDispatcher<F: Formatter> {
  formatter: F,
}

impl<F: Formatter> StdoutDispatcher<F> {
  pub fn new(formatter: F) -> Self {
    Self { formatter }
  }
}

impl<F: Formatter + Send + Sync> Dispatcher for StdoutDispatcher<F> {
  fn dispatch(&self, record: &Record) {
    let output = self.formatter.format(record);
    println!("{}", output);
  }
}

// 3. Stderr implementation
pub struct StderrDispatcher<F: Formatter> {
  formatter: F,
}

impl<F: Formatter> StderrDispatcher<F> {
  pub fn new(formatter: F) -> Self {
    Self { formatter }
  }
}

impl<F: Formatter + Send + Sync> Dispatcher for StderrDispatcher<F> {
  fn dispatch(&self, record: &Record) {
    let output = self.formatter.format(record);
    eprintln!("{}", output); // Outputs to Standard Error
  }
}
