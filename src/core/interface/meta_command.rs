use std::process::exit;

pub fn eval(command: &str) {
    match command {
        ".exit" => exit(0),
        _ => println!("Unrecognized command '{command}'")
    }    
}
