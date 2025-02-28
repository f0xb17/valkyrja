mod valkyrja;
fn main() {
    let file_error = valkyrja::throw_file_error("File not found!");
    let value_error = valkyrja::throw_value_error("Value not correct!");
    let runtime_error = valkyrja::throw_runtime_error("Error during run!");
    let unknown_error = valkyrja::throw_unkown_error("An unknown error occurred!");

    if valkyrja::returns_exception(&file_error) {
        println!("{}", valkyrja::get_error_message(file_error))
    }

    if valkyrja::returns_exception(&value_error) {
        println!("{}", valkyrja::get_error_message(value_error))
    }
    
    if valkyrja::returns_exception(&runtime_error) {
        println!("{}", valkyrja::get_error_message(runtime_error))
    }

    if valkyrja::returns_exception(&unknown_error) {
        println!("{}", valkyrja::get_error_message(unknown_error))
    }

    println!("{}", valkyrja::raise_info("Generation process started"));
    println!("{}", valkyrja::raise_warning("Couldn't read value x from y!"));
}
