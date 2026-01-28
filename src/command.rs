use std::str::FromStr;

pub(crate) enum Builtin {
    Exit,
    Echo,
    Type,
}

impl FromStr for Builtin {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Builtin::Exit),
            "echo" => Ok(Builtin::Echo),
            "type" => Ok(Builtin::Type),
            _ => Err(format!("{}: command not found", s)),
        }
    }
}