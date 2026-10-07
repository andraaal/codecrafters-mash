use std::{fs::File, io::BufRead, io::BufReader, io::Read, io::Write};

use crate::{HISTORY_FILE, ShellState};

pub(crate) fn save_history(state: &mut ShellState, file: Option<String>) {
    let file = file.or(std::env::var("HISTFILE").ok());
    if let Some(histfile) = file {
        let _ = state.rl.save_history(&histfile);

        // Remove versioning tag for CodeCrafters
        #[cfg(feature = "codecrafters")]
        remove_first_line(&histfile);
    } else {
        // No default history persistence for codecrafters
        #[cfg(not(feature = "codecrafters"))]
        let _ = state.rl.save_history(HISTORY_FILE);
    }
}

pub(crate) fn load_history(state: &mut ShellState, file: Option<String>) {
    let file = file.or(std::env::var("HISTFILE").ok());
    if let Some(histfile) = file {
        let _ = state.rl.load_history(&histfile);
    } else {
        // No default history persistence for codecrafters
        #[cfg(not(feature = "codecrafters"))]
        let _ = state.rl.load_history(HISTORY_FILE);
    }
}

fn remove_first_line(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Skip the first line
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    // Read the rest of the file
    let mut rest = Vec::new();
    reader.read_to_end(&mut rest)?;

    // Overwrite the file with the remaining content
    let mut out_file = File::create(path)?;
    out_file.write_all(&rest)?;

    Ok(())
}
