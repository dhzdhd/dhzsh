#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let segments = input.splitn(2, " ").collect::<Vec<&str>>();
        let command_opt = segments.first();

        const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

        if let Some(command) = command_opt {
            match command.trim() {
                "exit" => {
                    let code_opt = segments.get(1);
                    if let Some(code) = code_opt {
                        match code {
                            x if x.parse::<i32>().is_ok() => exit(x.parse::<i32>().unwrap()),
                            _ => println!("Invalid exit code"),
                        }
                    } else {
                        println!("Code not specified")
                    }
                }
                "echo" => {
                    let text_opt = segments.get(1);
                    if let Some(text) = text_opt {
                        println!("{}", text.trim())
                    }
                }
                "type" => {
                    let name_opt = segments.get(1);
                    if let Some(name) = name_opt {
                        match name.trim() {
                            x if BUILTINS.contains(&x) => {
                                println!("{} is a shell builtin", x)
                            }
                            x => println!("{}: not found", x),
                        }
                    } else {
                        println!("Command not specified")
                    }
                }
                x => println!("{}: command not found", x),
            }
        }
    }
}
