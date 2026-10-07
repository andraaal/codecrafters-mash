use faccess::PathExt;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::Validator;
use rustyline::{Changeset, Context, Helper};

/// Configures `rustyline` completion, hints, and validation for shell input.
pub(crate) struct ShellHelper {
    commands: Vec<String>,
    history_hinter: HistoryHinter,
    file_completer: FilenameCompleter,
}

impl ShellHelper {
    /// Creates a helper with the shell's built-in command list.
    pub(crate) fn new() -> Self {
        let mut commands = Vec::new();

        commands.push("exit".to_string());
        commands.push("echo".to_string());
        commands.push("type".to_string());
        commands.push("pwd".to_string());
        commands.push("cd".to_string());
        commands.push("history".to_string());
        commands.push("alias".to_string());
        commands.push("unalias".to_string());

        // Insert names of executables in PATH
        if let Some(paths) = get_executable_names() {
            commands.extend(paths);
        }

        commands.sort_unstable();

        ShellHelper {
            commands,
            history_hinter: HistoryHinter {},
            file_completer: FilenameCompleter::new(),
        }
    }

    /// Returns the command set so builtins and aliases can be registered.
    pub(crate) fn get_commands_mut(&mut self) -> &mut Vec<String> {
        &mut self.commands
    }
}

fn get_executable_names() -> Option<Vec<String>> {
    let path_var = std::env::var("PATH").ok()?;
    let mut result = Vec::new();

    for dir_path in std::env::split_paths(&path_var) {
        // Exclude windows paths on wsl to save time
        if dir_path.starts_with("/mnt") {
            continue;
        }

        for entry in std::fs::read_dir(dir_path).ok()? {
            let entry = entry.ok()?.path();
            if let Some(file) = entry.file_name()
                && entry.executable()
            {
                result.push(file.to_string_lossy().to_string());
            }
        }
    }
    Some(result)
}

impl Helper for ShellHelper {}
impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}

impl Hinter for ShellHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        if cfg!(feature = "codecrafters") {
            None
        } else {
            self.history_hinter.hint(line, pos, ctx)
        }
    }
}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let safe_pos = pos.min(line.len());
        let before = &line[..safe_pos];

        let start = before
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);

        let token = &line[start..safe_pos];
        let first_word = before[..start].trim().is_empty();

        if first_word {
            let mut out = Vec::new();
            for cmd in &self.commands {
                if cmd.starts_with(token) {
                    #[allow(unused_mut)]
                    let mut replace = cmd.clone();
                    let display = cmd.clone();

                    #[cfg(feature = "codecrafters")]
                    replace.push(' ');

                    out.push(Pair {
                        display: display,
                        replacement: replace,
                    });
                }
            }
            Ok((start, out))
        } else {
            self.file_completer.complete(line, pos, ctx)
        }
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        self.file_completer.update(line, start, elected, cl)
    }
}
