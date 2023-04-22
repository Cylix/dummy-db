use std::io::{self, Write};
use std::process::exit;

fn print_prompt() {
    print!("dummy-db> ");
    io::stdout().flush().unwrap_or_default();
}

fn read_input() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .unwrap_or_default();

    buffer.trim().to_string()
}

fn eval_command(command: &str) {
    match command {
        ".exit" => exit(0),
        _ => println!("Unrecognized command '{command}'")
    }
}

fn repl_loop() {
    loop {
        print_prompt();

        let command = read_input();
        eval_command(&command);
    }
}

fn main() {
    repl_loop();
}
