use std::env::current_dir;
use std::fs;
use std::io::{self, Write};

use std::process::Command;
use std::{collections::HashMap, env, path::PathBuf, process::exit};

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    glyph: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            glyph: "$".to_owned(),
        }
    }
}

lazy_static! {
    static ref CONFIG: Config = {
        let toml = fs::read_to_string("shell.conf.toml").unwrap_or(String::new());
        toml::from_str(toml.as_str()).unwrap_or_default()
    };

    #[derive(Debug)]
    static ref COMMAND_MAP: HashMap<String, PathBuf> = {
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
                                    let ext_str = ext.to_os_string().into_string().unwrap();
                                    let file_name_str = file_name.to_os_string().into_string().unwrap().replace(format!(".{ext_str}").as_str(), "");
                                    map.insert(file_name_str, path.clone());
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

fn change_directory(path: &str) {
    match path {
        "~" => {}
        x if x.starts_with(".") => {}
        x => {}
    }
}

fn main() {
    loop {
        print!("{} ", CONFIG.glyph);
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
                            x if COMMAND_MAP.contains_key(x) => {
                                println!("{x} is {}", COMMAND_MAP.get(x).unwrap().display())
                            }
                            x => println!("{}: not found", x),
                        }
                    } else {
                        println!("Command not specified")
                    }
                }
                "pwd" => match current_dir() {
                    Ok(dir) => println!("{}", dir.display()),
                    Err(_err) => println!("Unable to get current directory"),
                },
                "cd" => {
                    let path = segments.get(1).unwrap_or(&"");
                    change_directory(path)
                }
                x if COMMAND_MAP.contains_key(x) => {
                    match Command::new(x)
                        .arg(segments.get(1).unwrap_or(&"").trim())
                        .output()
                    {
                        Ok(res) => {
                            if res.stdout != [] {
                                println!(
                                    "{}",
                                    String::from_utf8(res.stdout).unwrap_or("None".to_owned()),
                                )
                            } else {
                                println!(
                                    "{}",
                                    String::from_utf8(res.stderr).unwrap_or("None".to_owned())
                                )
                            }
                        }
                        Err(_err) => println!("Failed to run process"),
                    }
                }
                x => println!("{}: command not found", x),
            }
        }
    }
}
