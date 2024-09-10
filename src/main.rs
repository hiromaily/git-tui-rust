// lib.rsの代わりにこのmain.rsにuse文をまとめても良い
use anyhow::{anyhow, Result};
use log::{debug, info};
use tokio::{sync::mpsc, task};

// local
use git_tui_rust::{git, input, logger, terminal, ui};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logger
    logger::init();
    info!("Start main()");

    // 現在のディレクトリを取得
    let repo_path_buf = std::env::current_dir()?;
    let repo_path = repo_path_buf
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert path to string"))?;
    debug!("Current directory: {:?}", repo_path); // e.g. /~/git-tui-rust

    // ブランチ一覧、現在のブランチを取得
    let branches = git::get_branches(repo_path)?;
    let mut current_branch = git::get_current_branch(repo_path)?;
    debug!("Branches: {:?}", branches); // ["dev", "main"]
    debug!("Current branch: {:?}", current_branch); // "dev"

    // ターミナル初期化
    let (mut terminal, alternate_screen) = terminal::init()?;
    info!("Terminal initialized.");

    // チャネルを使ってユーザーインプットを非同期に処理
    let (tx, mut rx) = mpsc::unbounded_channel();
    let handle = task::spawn(input::handle_events(tx));
    info!("Input handler task spawned.");

    // メッセージ変数
    let mut message = String::new();
    let mut selected_index = 0;

    let mut should_exit = false; // ここに明示的な終了フラグを追加

    while !should_exit {
        ui::draw(
            &mut terminal,
            &branches,
            &current_branch,
            &message,
            selected_index,
        )?;

        if let Some(event) = rx.recv().await {
            should_exit = input::process_event(
                event,
                &mut message,
                &mut selected_index,
                &branches,
                repo_path,
                &mut current_branch,
            )?;
        }
    }

    // ターミナル終了処理
    terminal::terminate(&mut terminal, alternate_screen)?;
    info!("Terminal terminated.");

    // 非同期タスクをキャンセルして安全に終了する
    handle.abort(); // タスクを中断する
    if let Err(err) = handle.await {
        if !err.is_cancelled() {
            eprintln!("Error while awaiting handle: {:?}", err);
        }
    }
    info!("Input handler task completed.");
    info!("Exiting main function.");

    Ok(())
}
