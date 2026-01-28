use std::str::FromStr;

pub(crate) enum Command {
    Exit,
    Echo,
    Type,
}

impl FromStr for Command {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Command::Exit),
            "echo" => Ok(Command::Echo),
            "type" => Ok(Command::Type),
            _ => Err(format!("{}: command not found", s)),
        }
    }
}