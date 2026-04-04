use crate::cmd::{IntStreamSource, IntStreamTarget};
use faccess::PathExt;
use std::io::{Error, PipeWriter};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) enum BuiltinType {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl FromStr for BuiltinType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(BuiltinType::Exit),
            "echo" => Ok(BuiltinType::Echo),
            "type" => Ok(BuiltinType::Type),
            "pwd" => Ok(BuiltinType::Pwd),
            "cd" => Ok(BuiltinType::Cd),
            _ => Err(()),
        }
    }
}

pub(crate) struct Builtin {
    typ: BuiltinType,
    args: Vec<String>,
    stdin_target: IntStreamSource,
    stdout_target: IntStreamTarget,
    stderr_target: IntStreamTarget,
}

pub(crate) struct BuiltinChild {
    inner: Builtin,
    stdout: String,
    stdin: String,
    stderr: String,
    executed: bool,
}

impl Builtin {
    pub(crate) fn new(typ: &str) -> Result<Self, ()> {
        let builtin_typ: BuiltinType = typ.parse()?;
        Ok(Builtin {
            typ: builtin_typ,
            args: Vec::new(),
            stdin_target: IntStreamSource::Inherit,
            stdout_target: IntStreamTarget::InheritStdout,
            stderr_target: IntStreamTarget::InheritStderr,
        })
    }

    pub(crate) fn set_stdout(&mut self, target: IntStreamTarget) {
        self.stdout_target = target;
    }

    pub(crate) fn set_stdin(&mut self, target: IntStreamSource) {
        self.stdin_target = target;
    }

    pub(crate) fn set_stderr(&mut self, target: IntStreamTarget) {
        self.stderr_target = target;
    }

    pub(crate) fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }

    pub(crate) fn spawn(self) -> Result<BuiltinChild, Error> {
        BuiltinChild::new(self)
    }
}

// TODO: Turn Builtin and BuiltinChild into one thing; executed differentiates them anyways
impl BuiltinChild {
    fn new(inner: Builtin) -> Result<Self, Error> {
        let mut child = BuiltinChild {
            inner,
            stdout: "".to_string(),
            stdin: "".to_string(),
            stderr: "".to_string(),
            executed: false,
        };
        // Note: No builtin actually uses stdin yet
        Ok(child)
    }

    pub(crate) fn execute(&mut self) -> Result<(), Error> {
        self.executed = true;
        match self.inner.typ {
            BuiltinType::Exit => std::process::exit(0),
            BuiltinType::Echo => {
                self.write_stdout(self.inner.args.join(" ").as_str())?;
            }
            BuiltinType::Pwd => {
                if let Ok(current) = std::env::current_dir() {
                    self.write_stdout(current.display().to_string().as_str())?;
                } else {
                    self.write_stderr("Current working directory either doesn't exist or you have insufficient privileges")?;
                };
            }
            BuiltinType::Cd => {
                if let Some(next) = self.inner.args.get(0) {
                    let target_path = &Self::create_path(&next);

                    if let Err(_) = std::env::set_current_dir(target_path) {
                        let message =
                            format!("cd: {}: No such file or directory", target_path.display());
                        self.write_stderr(&message)?;
                    }
                } else {
                    self.write_stderr("Cd requires at least one argument. If more than one are provided all but the first are discarded.")?;
                }
            }
            BuiltinType::Type => {
                if let Some(next) = self.inner.args.get(0) {
                    if BuiltinType::from_str(&next).is_ok() {
                        let message = format!("{} is a shell builtin", next);
                        self.write_stdout(&message)?;
                    } else {
                        if let Some(path) = Self::search_for_executable(&next) {
                            self.write_stdout(path.display().to_string().as_str())?;
                        } else {
                            let message = format!("{}: not found", next);
                            self.write_stdout(&message)?;
                        }
                    }
                } else {
                    self.write_stderr("Type requires at least one argument. If more than one are provided all but the first are discarded.")?;
                }
            }
        }
        Ok(())
    }

    fn write_stdout(&mut self, string: &str) -> Result<(), Error> {
        match self.inner.stdout_target {
            IntStreamTarget::InheritStdout => std::io::stdout().write_all(string.as_bytes())?,
            IntStreamTarget::InheritStderr => std::io::stderr().write_all(string.as_bytes())?,
            IntStreamTarget::Pipe => self.stdout.push_str(string),
            IntStreamTarget::Null => {}
            IntStreamTarget::Child(ref mut target) => {
                // SAFETY: Not really safe, this creates UB
                unsafe {
                    // TODO: This is undefined behavior and probably breaks things, I will fix it later
                    let mut writer: PipeWriter = std::mem::replace(target, std::mem::zeroed()).into();
                    writer.write_all(string.as_bytes())?;
                    let zero = std::mem::replace(target, writer.into());
                    std::mem::forget(zero);
                }
            }
        }

        Ok(())
    }

    fn write_stderr(&mut self, string: &str) -> Result<(), Error> {
        match self.inner.stderr_target {
            IntStreamTarget::InheritStdout => std::io::stdout().write_all(string.as_bytes())?,
            IntStreamTarget::InheritStderr => std::io::stderr().write_all(string.as_bytes())?,
            IntStreamTarget::Pipe => self.stderr.push_str(string),
            IntStreamTarget::Null => {}
            IntStreamTarget::Child(ref mut target) => {
                // SAFETY: Not really safe, this creates UB
                unsafe {
                    // TODO: This is undefined behavior and probably breaks things, I will fix it later
                    let mut writer: PipeWriter = std::mem::replace(target, std::mem::zeroed()).into();
                    writer.write_all(string.as_bytes())?;
                    let zero = std::mem::replace(target, writer.into());
                    std::mem::forget(zero);
                }
            }
        }

        Ok(())
    }

    fn search_for_executable(name: &str) -> Option<PathBuf> {
        let path_var = std::env::var("PATH").unwrap();

        for path_str in path_var.split(":") {
            let path = PathBuf::new().join(format!("{}/{}", path_str, name).as_str());
            if path.executable() {
                return Some(path);
            }
        }
        None
    }

    fn create_path(string: &str) -> PathBuf {
        let mut path = string.to_string();
        #[cfg(target_family = "unix")]
        {
            let home = std::env::var("HOME").unwrap_or_default();
            path = path.replace("~", home.as_str());
        }
        PathBuf::from(path)
    }

    pub(crate) fn write_to_stdin(&mut self, input: &str) -> Result<(), Error> {
        match self.inner.stdin_target {
            IntStreamSource::Pipe => {
                self.stdin.push_str(input);
                Ok(())
            }

            _ => Err(Error::new(ErrorKind::BrokenPipe, "This stdin is not piped")),
        }
    }

    pub(crate) fn get_stdout(&mut self) -> Result<String, Error> {
        match self.inner.stdout_target {
            IntStreamTarget::Pipe => {
                Ok(std::mem::take(&mut self.stdout))
            }

            _ => Err(Error::new(ErrorKind::BrokenPipe, "This stdout is not piped")),
        }

    }
    pub(crate) fn get_stderr(&mut self) -> Result<String, Error> {
        match self.inner.stderr_target {
            IntStreamTarget::Pipe => {
                Ok(std::mem::take(&mut self.stderr))
            }

            _ => Err(Error::new(ErrorKind::BrokenPipe, "This stderr is not piped")),
        }
    }

    pub(crate) fn redirect_stdout(&mut self, target: IntStreamTarget) -> Result<(), Error> {
        if matches!(target, IntStreamTarget::Pipe) {
            return Ok(());
        }
        match self.inner.stdout_target {
            IntStreamTarget::Pipe => {
                if !self.executed {
                    self.inner.stdout_target = target;
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "Can't redirect stdout: Builtin has already been executed"))
                }
            }
            _ => {
                Err(Error::new(ErrorKind::BrokenPipe, "Stdout is not piped"))
            }
        }
    }

    pub(crate) fn redirect_stderr(&mut self, target: IntStreamTarget) -> Result<(), Error> {
        if matches!(target, IntStreamTarget::Pipe) {
            return Ok(());
        }
        match self.inner.stderr_target {
            IntStreamTarget::Pipe => {
                if !self.executed {
                    self.inner.stderr_target = target;
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "Can't redirect stderr: Builtin has already been executed"))
                }
            }
            _ => {
                Err(Error::new(ErrorKind::BrokenPipe, "Stderr is not piped"))
            }
        }
    }

    pub(crate) fn redirect_stdin(&mut self, target: IntStreamSource) -> Result<(), Error> {
        if matches!(target, IntStreamSource::Pipe) {
            return Ok(());
        }
        match self.inner.stdin_target {
            IntStreamSource::Pipe => {
                if !self.executed {
                    self.inner.stdin_target = target;
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "Can't redirect stdin: Has already been written to"))
                }
            }
            _ => Err(Error::new(ErrorKind::BrokenPipe, "Stdin is not piped"))
        }
    }
}
