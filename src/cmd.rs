use std::cell::RefCell;
use crate::args::{Args, Token};
use crate::builtin::{Builtin, BuiltinChild};
use std::iter::Peekable;
use std::process::{Child, Command, Stdio};
use std::rc::Rc;
use std::io::Error;

pub(crate) struct Parser<'a> {
    tokens: Peekable<Args<'a>>,
    expressions: Vec<Expr>,
    errors: Vec<String>,
}

impl<'a> Parser<'_> {
    pub fn new(args: Peekable<Args<'a>>) -> Parser {
        Parser {
            tokens: args,
            expressions: Vec::new(),
            errors: Vec::new(),
        }
    }
    pub fn compile(mut self) -> Result<Vec<Expr>, Vec<String>> {
        let mut next = self.expression();
        self.expressions.push(next);
        while self.tokens.peek().is_some() {
            // TODO: Check for expression separator (\n) here
            next = self.expression();
            self.expressions.push(next);
        }
        if self.errors.is_empty() {
            Ok(self.expressions)
        } else {
            Err(self.errors)
        }
    }

    fn expression(&mut self) -> Expr {
        self.parse_precedence(0)
    }

    // TODO: implement precedence (and associativity)
    fn parse_precedence(&mut self, _min_prec: i32) -> Expr {
        if let Some(prefix_tk) = self.next_token() {
            let mut lhs;
            if let Some(parselet) = Self::prefix_parselet(&prefix_tk) {
                lhs = (parselet.parse)(self, prefix_tk);
            } else {
                self.errors.push("Invalid start of expression".to_string());
                return Expr::Error;
            }

            while let Some(infix_tk) = self.peek_token()
                && true
            {
                if let Some(parselet) = Self::infix_parselet(&infix_tk) {
                    let tk = self.next_token().unwrap();
                    lhs = (parselet.parse)(self, tk, lhs);
                } else {
                    break;
                }
            }
            lhs
        } else {
            self.errors.push("Invalid start of expression".to_string());
            Expr::Error
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn consume(&mut self, condition: fn(&Token) -> bool) -> bool {
        if self.tokens.peek().is_some_and(|tk| condition(tk)) {
            self.tokens.next();
            true
        } else {
            false
        }
    }

    fn consume_symbol(&mut self) -> Option<String> {
        if let Some(Token::Symbol(symbol)) = self.tokens.peek_mut() {
            let res = Some(std::mem::take(symbol));
            self.next_token();
            res
        } else {
            None
        }
    }

    // Any token that can't be at the start of an expression is considered infix
    const fn infix_parselet(tk: &Token) -> Option<InfixParselet> {
        let tp = match tk {
            Token::RedirectOutToFile => InfixParselet {
                precedence: 10,
                parse: |parser, _token, lhs| {
                    let rhs = parser.consume_symbol();
                    if let Some(right) = rhs {
                        Expr::RedirectOut(Box::new(lhs), right)
                    } else {
                        parser
                            .errors
                            .push("Expected filename after redirect".to_string());
                        Expr::Error
                    }
                },
            },
            _ => return None,
        };
        Some(tp)
    }

    // Any token that can start an expression is considered prefix
    const fn prefix_parselet(tk: &Token) -> Option<PrefixParselet> {
        let tp = match tk {
            Token::Symbol(_) => PrefixParselet {
                precedence: 10,
                parse: |parser, token| {
                    let mut arguments = Vec::new();
                    while let Some(arg) = parser.consume_symbol() {
                        arguments.push(arg);
                    }
                    let mut command = Cmd::new(&token.to_text());
                    command.set_args(&arguments);
                    Expr::Cmd(command)
                },
            },
            _ => return None,
        };
        Some(tp)
    }
}

type Precedence = u32;
struct PrefixParselet {
    precedence: Precedence,
    parse: fn(parser: &mut Parser, token: Token) -> Expr,
}

struct InfixParselet {
    precedence: Precedence,
    parse: fn(parser: &mut Parser, token: Token, lhs: Expr) -> Expr,
}

pub(crate) enum Expr {
    Cmd(Cmd),
    RedirectOut(Box<Expr>, String),
    Error, // Error is just here to be able to return something. I couldn't be bothered to write proper error handling (yet).
}


// Define the target of the streams here; then start the process to convert into a ChildProcess
pub(crate) enum Cmd {
    External(Command),
    Builtin(Builtin),
}

// Control the ChildProcess with wait(), ... and access piped streams
pub(crate) enum ChildProcess {
    ExternalChild(Child),
    BuiltinChild(BuiltinChild),
}

pub(crate) enum StreamTarget {
    InheritStdout, // Piped to the Stdout of the parent process
    InheritStderr, // Piped to the Stderr of the parent process
    Pipe, // Can be accessed in the child created by spawn
    Null, // To the void
    Child(Rc<RefCell<ChildProcess>>), // Piped to the Stdin of the child
}

pub(crate) enum StreamSource {
    Inherit, // Piped from the Stdin of the parent process
    Pipe, // Can be accessed in the child created by spawn
    Null, // To the void
    ChildStdout(Rc<RefCell<ChildProcess>>), // Piped from the Stdout of the child
    ChildStderr(Rc<RefCell<ChildProcess>>), // Piped from the Stdin of the child
}

impl Cmd {
    pub(crate) fn new(name: &str) -> Self {
        if let Ok(builtin) = Builtin::new(name) {
            Cmd::Builtin(builtin)
        } else {
            Cmd::External(Command::new(name))
        }
    }

    pub(crate)fn set_stdin(&mut self, target: StreamSource) -> Result<(), Error> {
        match self {
            Cmd::External(ref mut command) => {
                let stdio: Stdio = match target {
                    StreamSource::Inherit => Stdio::inherit(),
                    StreamSource::Pipe => Stdio::piped(),
                    StreamSource::Null => Stdio::null(),
                    StreamSource::ChildStdout(child) => {
                        let x = &mut *child.borrow_mut();
                        match x {
                            ChildProcess::ExternalChild(external) => {
                                external.stdout.take().into()
                            }
                            ChildProcess::BuiltinChild(_) => {}
                        }
                    }
                    StreamSource::ChildStderr(ref child) => {}
                };
                command.stdin(stdio);
            }
            Cmd::Builtin(_) => {}
        }
        Ok(())
    }

    pub(crate)fn set_stdout(&mut self, target: StreamTarget) -> Result<(), Error> {
        todo!()
    }

    pub(crate)fn set_stderr(&mut self, target: StreamTarget) -> Result<(), Error> {
        todo!()
    }

    pub(crate) fn spawn(self) -> Result<ChildProcess, Error> {
        todo!()
        // Note: Any stream that hasn't been defined ist inherit to find bugs and not deadlock anything
    }

    pub(crate) fn set_args(&mut self, args: &[String]) {
        todo!()
    }
}

impl ChildProcess {
    pub(crate) fn wait(&mut self) -> Result<(), Error> {
        todo!()
    }
    pub(crate) fn get_stdout(&mut self) -> Option<Box<dyn std::io::Read>> {
        todo!()
    }
    pub(crate) fn get_stderr(&mut self) -> Option<Box<dyn std::io::Read>> {
        todo!()
    }
    pub(crate) fn get_stdin(&mut self) -> Option<Box<dyn std::io::Write>> {
        todo!()
    }
}
