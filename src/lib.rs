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
  /// Formats the `ExceptionType` as a human-readable string.
  ///
  /// This method is part of the `fmt::Display` trait implementation.
  /// It matches the `ExceptionType` to its corresponding string
  /// representation and writes it to the formatter.
  ///
  /// # Arguments
  ///
  /// * `f` - A mutable reference to the formatter.
  ///
  /// # Returns
  ///
  /// * `fmt::Result` - The result of the write operation.
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
  /// Formats the `ExceptionLevel` as a human-readable string.
  ///
  /// This method is part of the `fmt::Display` trait implementation.
  /// It matches the `ExceptionLevel` to its corresponding string
  /// representation and writes it to the formatter.
  ///
  /// # Arguments
  ///
  /// * `f` - A mutable reference to the formatter.
  ///
  /// # Returns
  ///
  /// * `fmt::Result` - The result of the write operation.
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
  /// Creates a new `Exception` with the specified `exception_level`, `exception_type`, and `exception_message`.
  ///
  /// # Arguments
  ///
  /// * `exception_level` - The level of the exception being created.
  /// * `exception_type` - The type of exception being created.
  /// * `exception_message` - The message associated with the exception being created.
  fn new(exception_level: ExceptionLevel, exception_type: ExceptionType, exception_message: &str) -> Exception {
    Exception { exception_level, exception_type, exception_message }
  }

  /// Creates a new `Exception` with the specified `exception_type` and `exception_message`.
  ///
  /// The `exception_level` for all created exceptions is set to `ERROR`.
  ///
  /// # Arguments
  ///
  /// * `exception_type` - The type of exception being created.
  /// * `exception_message` - A message describing the exception.
  ///
  /// # Returns
  ///
  /// A new `Exception` instance with the specified type and message.
  pub fn create(exception_type: ExceptionType, exception_message: &str) -> Exception {
    match exception_type {
      ExceptionType::FILE => Exception::new(ExceptionLevel::ERROR, ExceptionType::FILE, exception_message),
      ExceptionType::VALUE => Exception::new(ExceptionLevel::ERROR, ExceptionType::VALUE, exception_message),
      ExceptionType::RUNTIME => Exception::new(ExceptionLevel::ERROR, ExceptionType::RUNTIME, exception_message),
      ExceptionType::UNKNOWN => Exception::new(ExceptionLevel::ERROR, ExceptionType::UNKNOWN, exception_message)
    }
  }

  /// Throw an exception. This will cause a panic, but it gives the context of the exception level and type.
  pub fn throw(&self) -> String {
    panic!("{} - {}: {}", ExceptionLevel::ERROR, self.exception_type, self.exception_message)
  }
}
