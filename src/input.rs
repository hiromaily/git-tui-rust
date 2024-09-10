use crate::git;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use log::debug;
use tokio::sync::mpsc::UnboundedSender;

pub async fn handle_events(tx: UnboundedSender<Event>) {
    loop {
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                tx.send(Event::Key(key)).unwrap();
                debug!("Key event sent: {:?}", key);
            }
        }
    }
}

pub fn process_event(
    event: Event,
    message: &mut String,
    selected_index: &mut usize,
    branches: &[String],
    repo_path: &str,
    current_branch: &mut String,
) -> Result<bool> {
    if let Event::Key(event) = event {
        debug!("Processing key event: {:?}", event.code);
        match event.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Up => {
                if *selected_index > 0 {
                    *selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if *selected_index < branches.len() - 1 {
                    *selected_index += 1;
                }
            }
            KeyCode::Enter => {
                let branch_name = &branches[*selected_index];
                if branch_name == current_branch {
                    *message = format!("This is the current branch: {}", branch_name);
                } else {
                    match git::checkout_branch(repo_path, branch_name) {
                        Ok(_) => {
                            *current_branch = git::get_current_branch(repo_path)?;
                            *message = format!("Branch was changed to: {}", current_branch);
                        }
                        Err(e) => {
                            *message = format!("Failed to checkout branch {}: {}", branch_name, e);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
