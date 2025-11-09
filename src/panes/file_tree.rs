use crate::git::{CommitMetadata, LineChangeType};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::collections::BTreeMap;

pub struct FileTreePane;

impl FileTreePane {
    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        metadata: Option<&CommitMetadata>,
        current_file_index: usize,
    ) {
        let block = Block::default()
            .title("File Tree")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let lines = if let Some(meta) = metadata {
            Self::build_tree_lines(meta, current_file_index)
        } else {
            vec![Line::from("No commit loaded")]
        };

        let content = Paragraph::new(lines).block(block);
        f.render_widget(content, area);
    }

    fn build_tree_lines(metadata: &CommitMetadata, current_file_index: usize) -> Vec<Line<'static>> {
        // Build directory tree
        let mut tree: BTreeMap<String, Vec<(usize, String, String, Color, usize, usize)>> = BTreeMap::new();

        for (index, change) in metadata.changes.iter().enumerate() {
            let (status_char, color) = match change.status.as_str() {
                "A" => ("+", Color::Green),
                "D" => ("-", Color::Red),
                "M" => ("~", Color::Yellow),
                "R" => (">", Color::Blue),
                _ => (" ", Color::White),
            };

            // Count additions and deletions
            let mut additions = 0;
            let mut deletions = 0;
            for hunk in &change.hunks {
                for line in &hunk.lines {
                    match line.change_type {
                        LineChangeType::Addition => additions += 1,
                        LineChangeType::Deletion => deletions += 1,
                        _ => {}
                    }
                }
            }

            let parts: Vec<&str> = change.path.split('/').collect();
            if parts.len() == 1 {
                // Root level file
                tree.entry("".to_string())
                    .or_insert_with(Vec::new)
                    .push((index, change.path.clone(), status_char.to_string(), color, additions, deletions));
            } else {
                // File in directory
                let dir = parts[..parts.len() - 1].join("/");
                let filename = parts[parts.len() - 1].to_string();
                tree.entry(dir)
                    .or_insert_with(Vec::new)
                    .push((index, filename, status_char.to_string(), color, additions, deletions));
            }
        }

        let mut lines = Vec::new();
        let sorted_dirs: Vec<_> = tree.keys().cloned().collect();

        for dir in sorted_dirs {
            let files = tree.get(&dir).unwrap();

            // Add directory header if not root
            if !dir.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{}/", dir),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                ]));
            }

            // Add files
            for (index, filename, status_char, color, additions, deletions) in files {
                let indent = if dir.is_empty() { "" } else { "  " }.to_string();
                let mut spans = vec![
                    Span::raw(indent),
                    Span::styled(
                        format!("{} ", status_char),
                        Style::default().fg(*color).add_modifier(Modifier::BOLD),
                    ),
                ];

                // Highlight current file
                if *index == current_file_index {
                    spans.push(Span::styled(
                        filename.clone(),
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(filename.clone()));
                }

                // Add change stats
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    format!("+{}", additions),
                    Style::default().fg(Color::Green),
                ));
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    format!("-{}", deletions),
                    Style::default().fg(Color::Red),
                ));

                lines.push(Line::from(spans));
            }
        }

        lines
    }
}
