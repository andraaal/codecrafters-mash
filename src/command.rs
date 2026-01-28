use std::str::FromStr;

pub(crate) enum Command {
    Exit,
    Echo,
}

impl FromStr for Command {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Command::Exit),
            "echo" => Ok(Command::Echo),
            _ => Err(format!("{}: command not found", s)),
        }
    }
}