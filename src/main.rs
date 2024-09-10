use anyhow::{anyhow, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::env;
use std::{io, process, time::Duration};
use tokio::{sync::mpsc, task};

//local
use git_tui_rust::{git, logger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logger
    logger::init();
    info!("start main()");

    // 現在のディレクトリを取得
    let repo_path_buf = env::current_dir()?;
    let repo_path = repo_path_buf
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert path to string"))?;

    // ブランチ一覧を取得
    let branches = git::get_branches(repo_path)?;

    // 現在のブランチを取得
    let mut current_branch = git::get_current_branch(repo_path)?;

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

    // メッセージ用の変数
    let mut message = String::new();

    // ブランチのリストを表示
    let mut selected_index = 0;
    loop {
        terminal.draw(|f| {
            let size = f.area(); // 修正された部分: size -> area
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(68), // 上部にブランチリスト
                        Constraint::Percentage(22), // 中央部にメッセージ表示用
                        Constraint::Percentage(10), // 下部に操作方法を表示
                    ]
                    .as_ref(),
                )
                .split(size);

            // ブランチ名とその強調表示を準備
            let branch_items: Vec<ListItem> = branches
                .iter()
                .enumerate()
                .map(|(i, b)| {
                    let mut branch_name = b.clone();
                    if b == &current_branch {
                        branch_name = format!("* {}", b);
                    }
                    if i == selected_index {
                        ListItem::new(Span::styled(
                            branch_name,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else {
                        ListItem::new(Span::raw(branch_name))
                    }
                })
                .collect();

            // ブランチリストのウィジェットを作成
            let list = List::new(branch_items)
                .block(Block::default().title("Branches").borders(Borders::ALL))
                .highlight_symbol(">> ");
            f.render_widget(list, chunks[0]);

            // メッセージ用のウィジェットを作成
            let message_block = Paragraph::new(message.clone())
                .block(Block::default().title("Message").borders(Borders::ALL));
            f.render_widget(message_block, chunks[1]);

            // 操作方法のメッセージを作成
            let help_message =
                Paragraph::new("Press 'q' to exit.").block(Block::default().borders(Borders::ALL));
            f.render_widget(help_message, chunks[2]);
        })?;

        if let Some(event) = rx.recv().await {
            match event.code {
                KeyCode::Char('q') => {
                    break;
                }
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
                    if branch_name == &current_branch {
                        message = format!("This is the current branch: {}", branch_name);
                        info!("This is the current branch: {}", branch_name);
                    } else {
                        match git::checkout_branch(repo_path, branch_name) {
                            Ok(_) => {
                                current_branch = git::get_current_branch(repo_path)?;
                                message = format!("Branch was changed to: {}", current_branch);
                                info!("Branch was changed to: {}", current_branch);
                            }
                            Err(e) => {
                                message =
                                    format!("Failed to checkout branch {}: {}", branch_name, e);
                                error!("Failed to checkout branch {}: {}", branch_name, e);
                            }
                        }
                    }
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

    process::exit(0); // プログラム自体を終了させる

    //Ok(())
}
