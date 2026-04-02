mod args;
mod builtin;
mod cmd;

use crate::args::{Args, Token};
use crate::builtin::BuiltinType;
use faccess::PathExt;
use std::io::{self, Write};
use std::iter::Peekable;
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
use std::str::FromStr;
use crate::cmd::{Expr, Parser};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            continue;
        }
        let words = Args::new(input.trim());

        execute(words);
    }
}

fn execute(args: Args) {
    let mut peek_args = args.peekable();
    let parser = Parser::new(peek_args);
    match parser.compile() {
        Ok(stmts) => {
            for stmt in stmts {
                match stmt {
                    Expr::Cmd(cmd) => {

                    }
                    Expr::RedirectOut(_, _) => {}
                    Expr::Error => panic!("Should not execute if there are any error tokens.")
                }
            }
        }
        Err(errors) => {
            errors.iter().for_each(|e| println!("{}", e));
        }
    }
    while let Some(arg) = peek_args.next() {
        match arg {
            Token::RedirectOutToFile => {
                //Interpret syntax
                todo!()
            }
            Token::Symbol(symbol) => {
                let input = std::iter::from_fn(|| take_next_symbol(&mut peek_args));
                execute_cmd(input);
            }
        }
    }
}

fn execute_cmd<T: Iterator<Item = String>>(mut words: T) {
    let cmd = words.next().unwrap_or_default();
    if let Ok(builtin) = cmd.parse() {
        execute_builtin(builtin, words);
    } else {
        let mut command = Command::new(&cmd);
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


fn take_next_symbol<I>(it: &mut Peekable<I>) -> Option<String>
where
    I: Iterator<Item = Token>,
{
    // Check the next item without consuming
    if matches!(it.peek(), Some(Token::Symbol(_))) {
        // Now consume and extract the owned value
        if let Some(Token::Symbol(s)) = it.next() {
            return Some(s);
        }
    }
    None
}
