#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        parse_command();
    }
}

fn parse_command() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = input.trim();

    match cmd {
        "exit" => std::process::exit(0),
        _ => println!("{}: command not found", input.trim()),
    }
}