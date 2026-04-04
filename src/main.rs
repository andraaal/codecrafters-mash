mod args;
mod builtin;
mod cmd;
mod pipe;

use crate::args::{Args, Token};
use crate::cmd::{ChildProcess, Cmd, Expr, Parser};
use faccess::PathExt;
use std::io::{self, Write};
use std::iter::Peekable;
use std::process::{Command, Stdio};
use std::str::FromStr;

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
                    execute_stmt(expr);
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

fn execute_stmt(stmt: Expr) {
        match stmt {
            Expr::Cmd(cmd) => {}
            Expr::RedirectOut(_, _) => {execute_cmd(stmt);}
            Expr::Error => panic!("Compiler's fault: Should not execute if there are any error tokens."),
            Expr::Pipe(_, _) => {
                execute_cmd(stmt);
            }
        }
}

fn execute_cmd(expr: Expr) -> Cmd {
    match expr {
        Expr::Cmd(cmd) => {cmd}
        Expr::RedirectOut(_, _) => {panic!("Not yet implemented")}
        Expr::Pipe(lhs, rhs) => {

        }
        Expr::Error => {}
    }
}