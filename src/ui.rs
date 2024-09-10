use crate::widgets;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::io;

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    branches: &[String],
    current_branch: &str,
    message: &str,
    selected_index: usize,
) -> Result<(), io::Error> {
    terminal
        .draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(68),
                        Constraint::Percentage(22),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(size);

            let branches_list =
                widgets::draw_branches_list(branches, current_branch, selected_index);
            f.render_widget(branches_list, chunks[0]);

            let message_paragraph = widgets::draw_message_paragraph(message);
            f.render_widget(message_paragraph, chunks[1]);

            let help_paragraph = widgets::draw_help_paragraph();
            f.render_widget(help_paragraph, chunks[2]);
        })
        .map(|_| ()) // ここで CompletedFrame を () に変換
}
