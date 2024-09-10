use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// retutned type of init()
type TerminalResult =
    Result<(Terminal<CrosstermBackend<io::Stdout>>, AlternateScreen), Box<dyn std::error::Error>>;

pub struct AlternateScreen;

/// Initiate Terminal
///
/// 1. Enable Raw Mode:
///    Switches the terminal into raw mode for easier user input.
/// 2. Enable Alternate Screen:
///    Displays a dedicated screen for this application apart from the normal terminal screen.
///    This makes it easier to draw the TUI (Terminal User Interface).
///
pub fn init() -> TerminalResult {
    debug!("terminal::init(tx)");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok((terminal, AlternateScreen))
}

/// Terminate Terminal
///
/// 1. Disable Raw Mode:
///    Reverts the terminal to its original mode when the program exits.
/// 2. Disable Alternate Screen:
///    Redisplays the original terminal screen when the program exits.
/// 3. Show Cursor:
///    Redisplays the cursor when the program exits.
///
pub fn terminate(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    _alternate_screen: AlternateScreen,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("terminal::terminate()");

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
