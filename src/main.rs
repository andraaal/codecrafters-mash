mod builtin;

use crate::builtin::Builtin;
use faccess::PathExt;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
use std::str::{FromStr, SplitWhitespace};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            continue;
        }

        execute(input);
    }
}

fn execute(raw: String) {
    let mut words = raw.trim().split_whitespace();

    let cmd = words.next().unwrap_or("");
    if let Ok(builtin) = cmd.parse() {
        execute_builtin(builtin, words);
    } else {
        let mut command = Command::new(cmd);
        for arg in words {
            command.arg(arg);
        }
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command.stdin(Stdio::inherit());
        if command.status().is_err() {
            println!("{}: command not found", cmd);
        }
    }
}

fn execute_builtin(cmd: Builtin, mut args: SplitWhitespace) {
    match cmd {
        Builtin::Exit => exit(0),
        Builtin::Echo => println!("{}", args.collect::<Vec<_>>().join(" ")),
        Builtin::Pwd => {
            if let Ok(current) = std::env::current_dir() {
                println!("{}", current.display());
            } else {
                println!(
                    "Current working directory either doesn't exist or you have insufficient privileges"
                );
            };
        }
        Builtin::Cd => {
            if let Some(next) = args.next() {
                let target_path = &create_path(next);

                if let Err(_) = std::env::set_current_dir(target_path) {
                    println!("cd: {}: No such file or directory", target_path.display());
                }
            } else {
                println!(
                    "Cd requires at least one argument. If more than one are provided all but the first are discarded."
                );
            }
        }
        Builtin::Type => {
            if let Some(next) = args.next() {
                if Builtin::from_str(next).is_ok() {
                    println!("{} is a shell builtin", next);
                } else {
                    if let Some(path) = search_for_executable(next) {
                        println!("{}", path.display());
                    } else {
                        println!("{}: not found", next);
                    }
                }
            } else {
                println!(
                    "Type requires at least one argument. If more than one are provided all but the first are discarded."
                );
            }
        }
    }
}

fn search_for_executable(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var("PATH").unwrap();

    for path_str in path_var.split(":") {
        let path = PathBuf::new().join(format!("{}/{}", path_str, name).as_str());
        if path.executable() {
            return Some(path);
        }
    }
    None
}

fn create_path(string: &str) -> PathBuf {
    let mut path = string.to_string();
    #[cfg(target_family = "unix")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        path = path.replace("~", home.as_str());
    }
    PathBuf::from(path)
}
