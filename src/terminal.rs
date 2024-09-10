use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// 型エイリアスの定義
type TerminalResult =
    Result<(Terminal<CrosstermBackend<io::Stdout>>, AlternateScreen), Box<dyn std::error::Error>>;

pub struct AlternateScreen;

pub fn init() -> TerminalResult {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok((terminal, AlternateScreen))
}

pub fn terminate(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    _alternate_screen: AlternateScreen,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
