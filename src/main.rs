mod command;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{exit};
use std::str::SplitWhitespace;
use crate::command::Command;

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

fn execute(cmd: Command, args: SplitWhitespace) {
    match cmd {
        Command::Exit => exit(0),
        Command::Echo => println!("{}", args.collect::<Vec<_>>().join(" ")),
    }
}