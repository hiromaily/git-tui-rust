use anyhow::{anyhow, Result};
use git2::{BranchType, Repository};
use std::env;

fn get_branches(repo_path: &str) -> Result<Vec<String>> {
    let repo = Repository::open(repo_path)?;
    let branches = repo
        .branches(Some(BranchType::Local))?
        .filter_map(|branch| branch.ok())
        .filter_map(|(branch, _)| branch.name().ok().flatten().map(|s| s.to_string()))
        .collect();
    Ok(branches)
}

fn checkout_branch(repo_path: &str, branch_name: &str) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    let (object, reference) = repo.revparse_ext(branch_name)?;
    repo.checkout_tree(&object, None)?;
    if let Some(gref) = reference {
        repo.set_head(gref.name().unwrap())?;
    } else {
        repo.set_head_detached(object.id())?;
    }
    Ok(())
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::{io, time::Duration};
use tokio::{sync::mpsc, task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 現在のディレクトリを取得
    let repo_path_buf = env::current_dir()?;
    let repo_path = repo_path_buf
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert path to string"))?;

    // ブランチ一覧を取得
    let branches = get_branches(repo_path)?;

    // ターミナルの初期化
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // チャネルを使ってユーザーインプットを非同期に処理
    let (tx, mut rx) = mpsc::unbounded_channel();
    task::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    tx.send(key).unwrap();
                }
            }
        }
    });

    // ブランチのリストを表示
    let mut selected_index = 0;
    loop {
        terminal.draw(|f| {
            let size = f.area(); // size を area に変更
            let block = Block::default().title("Branches").borders(Borders::ALL);
            let branch_items: Vec<ListItem> = branches
                .iter()
                .map(|b| ListItem::new(Span::raw(b)))
                .collect();
            let list = List::new(branch_items)
                .block(block)
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_widget(list, size);
        })?;

        if let Some(event) = rx.recv().await {
            match event.code {
                KeyCode::Char('q') => break,
                #[allow(clippy::implicit_saturating_sub)]
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < branches.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    let branch_name = &branches[selected_index];
                    checkout_branch(repo_path, branch_name)?;
                }
                _ => {}
            }
        }
    }

    // 終了処理
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
