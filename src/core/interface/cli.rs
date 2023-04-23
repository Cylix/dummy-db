use std::io::{self, Write};
use super::{meta_command, sql_command};

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
    if command.is_empty() {
        return
    }

    match command.chars().next().unwrap() {
        '.' => meta_command::eval(command),
        _ => sql_command::eval(command)
    }
}

pub fn repl_loop() {
    loop {
        print_prompt();

        let command = read_input();
        eval_command(&command);
    }
}
