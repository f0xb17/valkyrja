use std::fmt;
pub enum ExceptionLevel {
  INFO,
  WARNING,
  ERROR
}

pub enum ExceptionType {
  FILE,
  VALUE,
  RUNTIME,
  UNKNOWN
}

pub struct Exception<'a> {
  exception_level: ExceptionLevel,
  exception_type: ExceptionType,
  exception_message: &'a str
}

impl fmt::Display for ExceptionType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self {
      ExceptionType::FILE => "FILE",
      ExceptionType::VALUE => "VALUE",
      ExceptionType::RUNTIME => "RUNTIME",
      ExceptionType::UNKNOWN => "UNKNOWN",
    };
    write!(f, "{}", s)
  }
}

impl fmt::Display for ExceptionLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self {
      ExceptionLevel::INFO => "INFO",
      ExceptionLevel::WARNING => "WARNING",
      ExceptionLevel::ERROR => "ERROR",
    };
    write!(f, "{}", s)
  }
}
impl<'a> Exception<'a> {
  fn new(exception_level: ExceptionLevel, exception_type: ExceptionType, exception_message: &str) -> Exception {
    Exception { exception_level, exception_type, exception_message }
  }

  pub fn create(exception_type: ExceptionType, exception_message: &str) -> Exception {
    match exception_type {
      ExceptionType::FILE => Exception::new(ExceptionLevel::ERROR, ExceptionType::FILE, exception_message),
      ExceptionType::VALUE => Exception::new(ExceptionLevel::ERROR, ExceptionType::VALUE, exception_message),
      ExceptionType::RUNTIME => Exception::new(ExceptionLevel::ERROR, ExceptionType::RUNTIME, exception_message),
      ExceptionType::UNKNOWN => Exception::new(ExceptionLevel::ERROR, ExceptionType::UNKNOWN, exception_message)
    }
  }

  pub fn throw(&self) -> String {
    panic!("{} - {}: {}", ExceptionLevel::ERROR, self.exception_type, self.exception_message)
  }
}
