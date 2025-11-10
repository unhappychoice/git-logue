use crate::git::CommitMetadata;
use crate::theme::Theme;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};

pub struct StatusBarPane;

impl StatusBarPane {
    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        metadata: Option<&CommitMetadata>,
        theme: &Theme,
    ) {
        let block = Block::default()
            .style(Style::default().bg(theme.background_left))
            .padding(Padding {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            });

        let status_text = if let Some(meta) = metadata {
            let hash_short = &meta.hash[..7.min(meta.hash.len())];
            let date_str = meta.date.format("%Y-%m-%d %H:%M:%S").to_string();

            let mut lines = vec![
                Line::from(vec![
                    Span::raw("hash: "),
                    Span::styled(hash_short, Style::default().fg(theme.status_hash)),
                ]),
                Line::from(vec![
                    Span::raw("author: "),
                    Span::styled(&meta.author, Style::default().fg(theme.status_author)),
                ]),
                Line::from(vec![
                    Span::raw("date: "),
                    Span::styled(date_str, Style::default().fg(theme.status_date)),
                ]),
            ];

            // Add commit message lines (skip empty lines)
            for msg_line in meta.message.lines() {
                if !msg_line.trim().is_empty() {
                    lines.push(Line::from(vec![Span::styled(
                        msg_line,
                        Style::default().fg(theme.status_message),
                    )]));
                }
            }

            lines
        } else {
            vec![Line::from(vec![Span::styled(
                "No commit loaded",
                Style::default().fg(theme.status_no_commit),
            )])]
        };

        let content = Paragraph::new(status_text)
            .block(block)
            .wrap(Wrap { trim: false });

        f.render_widget(content, area);
    }
}
