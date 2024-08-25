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
        x => println!("{}: command not found", x.unwrap()),
    }
}
