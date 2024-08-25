#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut segments = input.split_whitespace();

    match segments.next() {
        Some(command) => println!("{}: command not found", command),
        None => {}, // Do nothing if there's no command (e.g. just ENTER was pressed)
    }
}
