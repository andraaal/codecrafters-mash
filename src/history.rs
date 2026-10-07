use crate::{HISTORY_FILE, ShellState};

pub(crate) fn save_history(state: &mut ShellState, file: Option<String>) {
    let file = file.or(std::env::var("HISTFILE").ok());
    if let Some(histfile) = file {
        let _ = state.rl.save_history(&histfile);
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
