use valkyrja::{Exception, ExceptionType};


fn main() {
    Exception::create(ExceptionType::FILE, "File not found").throw();
}