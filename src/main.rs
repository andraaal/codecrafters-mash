mod args;
mod builtin;
mod cmd;

use crate::args::Args;
use crate::cmd::{Cmd, Expr, Parser, StreamSource, StreamTarget};
use std::io::{self, Write};

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
        let peek_args = words.peekable();
        let parser = Parser::new(peek_args);
        match parser.compile() {
            Ok(exprs) => {
                for expr in exprs {
                    execute(expr).unwrap().spawn().unwrap();
                }
            }
            Err(errs) => {
                for err in errs {
                    eprintln!("{}", err);
                }
            }
        }
    }
}

fn execute(stmt: Expr) -> Result<Cmd, std::io::Error> {
    match stmt {
        Expr::Cmd(cmd) => {
            Ok(cmd)
        }
        Expr::RedirectOut(_, _) => {
            todo!()
        }
        Expr::Error => {
            panic!("Compiler's fault: Should not execute if there are any error tokens.")
        }
        Expr::Pipe(lhs, rhs) => {
            let mut left_cmd = execute(*lhs)?;
            let mut right_cmd = execute(*rhs)?;
            left_cmd.set_stdout(StreamTarget::Child(&mut right_cmd))?;
            left_cmd.spawn()?;
            Ok(right_cmd)
        }
    }
}


