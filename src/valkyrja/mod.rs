pub struct Exception {
  level: ExceptionLevel,
  code: ExceptionCode,
  message: String
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

fn get_exception_level(level: ExceptionLevel) -> String {
  match level {
    ExceptionLevel::INFO => return String::from("INFO"),
    ExceptionLevel::ERROR => return String::from("ERROR"),
    ExceptionLevel::WARNING => return String::from("WARNING")
  }
}

fn get_exception_code(code: ExceptionCode) -> String {
  match code {
    ExceptionCode::FileError => return String::from("FILE ERROR"),
    ExceptionCode::ValueError => return String::from("VALUE ERROR"),
    ExceptionCode::RuntimeError => return String::from("RUNTIME ERROR"),
    ExceptionCode::UnkownError => return String::from("UNKOWN ERROR"),
  }
}

fn create_exception(level: ExceptionLevel, code: ExceptionCode, message: &str) -> Exception {
  return Exception{ level: level, code: code, message: String::from(message) }
}

pub fn throw_file_error(message: &str) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::FileError, message)
}

pub fn throw_value_error(message: &str) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::ValueError, message)
}

pub fn throw_runtime_error(message: &str) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::RuntimeError, message)
}

pub fn throw_unkown_error(message: &str) -> Exception {
  return create_exception(ExceptionLevel::ERROR, ExceptionCode::UnkownError, message)
}

pub fn get_error_message(e: Exception) -> String {
  return format!("{} - {}: {}", &get_exception_level(e.level), &get_exception_code(e.code), &e.message)
}

pub fn raise_info(message: &str) -> String {
  return format!("{}: {}", &get_exception_level(ExceptionLevel::INFO), message)
}

pub fn raise_warning(message: &str) -> String {
  return format!("{}: {}", &get_exception_level(ExceptionLevel::WARNING), message)
}
