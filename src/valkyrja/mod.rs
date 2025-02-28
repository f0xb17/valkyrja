pub struct Exception {
  level: ExceptionLevel,
  code: ExceptionCode,
  message: String
}

enum ExceptionLevel {
  INFO,
  WARNING,
  ERROR
}

enum ExceptionCode {
  FileError,
  ValueError,
  RuntimeError,
  UnkownError
}

fn get_level(level: ExceptionLevel) -> i8 {
  match level {
    ExceptionLevel::INFO => return 0,
    ExceptionLevel::WARNING => return 1,
    ExceptionLevel::ERROR => return 2
  }
}

fn get_code(code: ExceptionCode) -> i8 {
  match code {
    ExceptionCode::FileError => return 0,
    ExceptionCode::ValueError => return 1,
    ExceptionCode::RuntimeError => return 2,
    ExceptionCode::UnkownError => return 3
  }
}

fn create_exception(level: ExceptionLevel, code: ExceptionCode, message: &str) -> Exception {
  return Exception{ level: level, code: code, message: message.to_string() }
}

pub fn returns_exception(e: Exception) -> bool {
  return get_level(e.level) != -1 && get_code(e.code) != -1 && e.message != "".to_string()
}

