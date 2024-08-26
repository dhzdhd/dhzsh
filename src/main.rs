use std::ffi::OsString;
use std::io::{self, Write};
use std::{collections::HashMap, env, path::PathBuf, process::exit};

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref COMMAND_MAP: HashMap<OsString, PathBuf> = {
        let mut map = HashMap::new();

        if let Ok(paths) = get_env_paths() {
            for path in paths {
                if let Ok(dir) = path.read_dir() {
                    for dir_entry in dir {
                        if let Ok(file) = dir_entry {
                            let file_path = file.path().clone();
                            let file_name = file_path.file_name().unwrap();
                            let file_ext = file_path.extension();

                            if let Some(ext) = file_ext {
                                if ext == "exe" {
                                    map.insert(file_name.to_os_string(), path.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        map
    };
}

fn get_env_paths() -> Result<Vec<PathBuf>, env::VarError> {
    return env::var("PATH").map(|raw_paths| {
        let paths = raw_paths.clone();

        let vec = paths
            .split(";")
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>();
        vec
    });
}

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
                            x if COMMAND_MAP.contains_key(&OsString::from(x)) => {
                                println!(
                                    "{x} is {}",
                                    COMMAND_MAP.get(&OsString::from(x)).unwrap().display()
                                )
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
