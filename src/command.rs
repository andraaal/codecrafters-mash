use std::str::FromStr;

pub(crate) enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
}

impl FromStr for Builtin {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Builtin::Exit),
            "echo" => Ok(Builtin::Echo),
            "type" => Ok(Builtin::Type),
            "pwd" => Ok(Builtin::Pwd),
            _ => Err(format!("{}: command not found", s)),
        }
    }
}