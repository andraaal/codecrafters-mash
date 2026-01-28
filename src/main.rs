mod command;

use crate::command::Command;
use faccess::PathExt;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;
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

        parse_command(input);
    }
}

fn parse_command(raw: String) {
    let mut words = raw.trim().split_whitespace();

    let cmd = words.next().unwrap_or("").parse();

    match cmd {
        Ok(cmd) => execute(cmd, words),
        Err(msg) => println!("{}", msg),
    }
}

fn execute(cmd: Command, mut args: SplitWhitespace) {
    match cmd {
        Command::Exit => exit(0),
        Command::Echo => println!("{}", args.collect::<Vec<_>>().join(" ")),
        Command::Type => {
            if let Some(next) = args.next() {
                if Command::from_str(next).is_ok() {
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
