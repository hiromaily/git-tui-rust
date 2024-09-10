use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

// ブランチリストを描画する関数
pub fn draw_branches_list<'a>(
    branches: &'a [String],
    current_branch: &'a str,
    selected_index: usize,
) -> List<'a> {
    let branch_items: Vec<ListItem> = branches
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let mut branch_name = b.clone();
            if b == current_branch {
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

    List::new(branch_items)
        .block(Block::default().title("Branches").borders(Borders::ALL))
        .highlight_symbol(">> ")
}

// メッセージを描画する関数
pub fn draw_message_paragraph(message: &str) -> Paragraph {
    Paragraph::new(message.to_string())
        .block(Block::default().title("Message").borders(Borders::ALL))
}

// 操作方法を描画する関数
pub fn draw_help_paragraph<'a>() -> Paragraph<'a> {
    Paragraph::new("Press 'q' to exit.").block(Block::default().borders(Borders::ALL))
}
