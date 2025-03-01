mod valkyrja;
use valkyrja::{throw_file_exception, throw_runtime_exception, throw_unkown_exception, 
                throw_value_exception, Exception, 
                get_exception_message, raise_info, raise_warning};

fn divide(x: u8, y: u8) -> Result<u8, Exception> {
    if y == 0 {
        return Err(throw_value_exception("Divison by Zero is not allowed!"));
    }
    Ok(x / y)
}

fn main() {
    let _file_error = throw_file_exception("File not found!");
    let _value_error = throw_value_exception("Value not correct!");
    let _runtime_error = throw_runtime_exception("Error during run!");
    let _unknown_error = throw_unkown_exception("An unknown error occurred!");

    println!("{}", raise_info("Generation process started"));
    println!("{}", raise_warning("Couldn't read value x from y!"));

    let result = divide(2,0 );
    match result {
        Ok(value) => println!("The Result ist: {}", value),
        Err(e) => println!("{}", get_exception_message(e))
    }
}
