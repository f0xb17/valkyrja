pub struct Exception {
  level:    ExceptionLevel,
  code:     ExceptionCode,
  message:  String
}

#[derive(Copy, Clone)]
enum ExceptionLevel {
  INFO,
  WARNING,
  ERROR
}

#[derive(Copy, Clone)]
enum ExceptionCode {
  FileError,
  ValueError,
  RuntimeError,
  UnkownError
}

/// Converts an `ExceptionLevel` enum into a descriptive string representation.
/// 
/// # Arguments
/// 
/// * `level` - The severity level of the exception, represented by an `ExceptionLevel`.
/// 
/// # Returns
/// 
/// A `String` containing the descriptive representation of the exception level.
fn get_exception_level(level: ExceptionLevel) -> String {
  match level {
    ExceptionLevel::INFO => return String::from("INFO"),
    ExceptionLevel::ERROR => return String::from("ERROR"),
    ExceptionLevel::WARNING => return String::from("WARNING")
  }
}

/// Converts an `ExceptionCode` enum into a descriptive string representation.
/// 
/// # Arguments
/// 
/// * `code` - The specific code of the exception, represented by an `ExceptionCode`.
/// 
/// # Returns
/// 
/// A `String` containing the descriptive representation of the exception code.
fn get_exception_code(code: ExceptionCode) -> String {
  match code {
    ExceptionCode::FileError => return String::from("FILE ERROR"),
    ExceptionCode::ValueError => return String::from("VALUE ERROR"),
    ExceptionCode::RuntimeError => return String::from("RUNTIME ERROR"),
    ExceptionCode::UnkownError => return String::from("UNKOWN ERROR"),
  }
}

/// Creates a new `Exception` with the specified level, code, and message.
/// 
/// # Arguments
/// 
/// * `level` - The severity level of the exception, represented by an `ExceptionLevel`.
/// * `code` - The specific code of the exception, represented by an `ExceptionCode`.
/// * `message` - A descriptive message providing additional details about the exception.
/// 
/// # Returns
/// 
/// An `Exception` struct containing the provided level, code, and message.
fn create_exception(level: ExceptionLevel, code: ExceptionCode, message: &str) -> Exception {
  return Exception{ level: level, code: code, message: String::from(message) }
}

/// Throws a file exception with the specified message.
/// 
/// # Arguments
/// 
/// * `message` - A descriptive message providing additional details about the exception.
/// 
/// # Returns
/// 
/// An `Exception` struct containing the provided message and the ERROR level and FILE ERROR code.
pub fn throw_file_exception<T: Into<Option<&'static str>>>(message: T) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::FileError, message.into().unwrap_or("Encountered a file error!"))
}

/// Throws a value exception with the specified message.
/// 
/// # Arguments
/// 
/// * `message` - A descriptive message providing additional details about the exception.
/// 
/// # Returns
/// 
/// An `Exception` struct containing the provided message and the ERROR level and VALUE ERROR code.
pub fn throw_value_exception<T: Into<Option<&'static str>>>(message: T) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::ValueError, message.into().unwrap_or("Encountered a value error!"))
}

/// Throws a runtime exception with the specified message.
/// 
/// # Arguments
/// 
/// * `message` - A descriptive message providing additional details about the exception.
/// 
/// # Returns
/// 
/// An `Exception` struct containing the provided message and the ERROR level and RUNTIME ERROR code.
pub fn throw_runtime_exception<T: Into<Option<&'static str>>>(message: T) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::RuntimeError, message.into().unwrap_or("Encountered a runtime error!"))
}

/// Throws an unknown exception with the specified message.
/// 
/// # Arguments
/// 
/// * `message` - A descriptive message providing additional details about the exception.
/// 
/// # Returns
/// 
/// An `Exception` struct containing the provided message and the ERROR level and UNKOWN ERROR code.
pub fn throw_unkown_exception<T: Into<Option<&'static str>>>(message: T) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::UnkownError, message.into().unwrap_or("Encountered an unkown error!"))
}

/// Formats the exception details into a readable string representation.
///
/// # Arguments
///
/// * `e` - An `Exception` instance containing the level, code, and message of the exception.
///
/// # Returns
///
/// A `String` in the format of "LEVEL - CODE: message", where `LEVEL` is the exception's severity level,
/// `CODE` is the specific exception code, and `message` is the descriptive message.

pub fn get_exception_message(e: Exception) -> String {
  return format!("{} - {}: {}", &get_exception_level(e.level), &get_exception_code(e.code), &e.message)
}

/// Formats a string into a readable info message.
///
/// # Arguments
///
/// * `message` - A descriptive message providing additional details about the information.
///
/// # Returns
///
/// A `String` in the format of "INFO: message", where `message` is the descriptive message.
pub fn raise_info(message: &str) -> String {
  return format!("{}: {}", &get_exception_level(ExceptionLevel::INFO), message)
}

/// Formats a string into a readable warning message.
///
/// # Arguments
///
/// * `message` - A descriptive message providing additional details about the warning.
///
/// # Returns
///
/// A `String` in the format of "WARNING: message", where `message` is the descriptive message.
pub fn raise_warning(message: &str) -> String {
  return format!("{}: {}", &get_exception_level(ExceptionLevel::WARNING), message)
}


/// Formats a string into a readable error message.
///
/// # Arguments
///
/// * `message` - A descriptive message providing additional details about the error.
///
/// # Returns
///
/// A `String` in the format of "ERROR: message", where `message` is the descriptive message.
pub fn raise_error(message: &str) -> String {
  return format!("{}: {}", &get_exception_level(ExceptionLevel::ERROR), message)
}
