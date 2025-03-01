mod valkyrja;
use valkyrja::{throw_file_error, throw_runtime_error, throw_unkown_error, 
                throw_value_error, Exception, 
                get_error_message, raise_info, raise_warning};

fn divide(x: u8, y: u8) -> Result<u8, Exception> {
    if y == 0 {
        return Err(throw_value_error("Divison by Zero is not allowed!"));
    }
    Ok(x / y)
}

fn main() {
    let _file_error = throw_file_error("File not found!");
    let _value_error = throw_value_error("Value not correct!");
    let _runtime_error = throw_runtime_error("Error during run!");
    let _unknown_error = throw_unkown_error("An unknown error occurred!");

    println!("{}", raise_info("Generation process started"));
    println!("{}", raise_warning("Couldn't read value x from y!"));

    let result = divide(2,0 );
    match result {
        Ok(value) => println!("The Result ist: {}", value),
        Err(e) => println!("{}", get_error_message(e))
    }
}
