mod valkyrja;

fn divide(x: u8, y: u8) -> Result<u8, valkyrja::Exception> {
    if y == 0 {
        return Err(valkyrja::throw_value_error("Divison by Zero is not allowed!"));
    }
    Ok(x / y)
}

fn main() {
    let _file_error = valkyrja::throw_file_error("File not found!");
    let _value_error = valkyrja::throw_value_error("Value not correct!");
    let _runtime_error = valkyrja::throw_runtime_error("Error during run!");
    let _unknown_error = valkyrja::throw_unkown_error("An unknown error occurred!");

    println!("{}", valkyrja::raise_info("Generation process started"));
    println!("{}", valkyrja::raise_warning("Couldn't read value x from y!"));

    let result = divide(2,0 );
    match result {
        Ok(value) => println!("The Result ist: {}", value),
        Err(e) => println!("{}", valkyrja::get_error_message(e))
    }
}
