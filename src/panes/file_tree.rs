use crate::git::{CommitMetadata, LineChangeType};
use crate::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};
use std::collections::BTreeMap;

type FileEntry = (usize, String, String, Color, usize, usize);
type FileTree = BTreeMap<String, Vec<FileEntry>>;

pub struct FileTreePane;

impl FileTreePane {
    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        metadata: Option<&CommitMetadata>,
        current_file_index: usize,
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

        let lines = if let Some(meta) = metadata {
            // Subtract horizontal padding (2 on each side)
            let content_width = area.width.saturating_sub(4) as usize;
            Self::build_tree_lines(meta, current_file_index, theme, content_width)
        } else {
            vec![Line::from("No commit loaded")]
        };

        let content = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });
        f.render_widget(content, area);
    }

    fn build_tree_lines(
        metadata: &CommitMetadata,
        current_file_index: usize,
        theme: &Theme,
        area_width: usize,
    ) -> Vec<Line<'static>> {
        // Build directory tree
        let mut tree: FileTree = BTreeMap::new();

        for (index, change) in metadata.changes.iter().enumerate() {
            let (status_char, color) = match change.status.as_str() {
                "A" => ("+", theme.file_tree_added),
                "D" => ("-", theme.file_tree_deleted),
                "M" => ("~", theme.file_tree_modified),
                "R" => (">", theme.file_tree_renamed),
                _ => (" ", theme.file_tree_default),
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
                tree.entry("".to_string()).or_default().push((
                    index,
                    change.path.clone(),
                    status_char.to_string(),
                    color,
                    additions,
                    deletions,
                ));
            } else {
                // File in directory
                let dir = parts[..parts.len() - 1].join("/");
                let filename = parts[parts.len() - 1].to_string();
                tree.entry(dir).or_default().push((
                    index,
                    filename,
                    status_char.to_string(),
                    color,
                    additions,
                    deletions,
                ));
            }
        }

        let mut lines = Vec::new();
        let sorted_dirs: Vec<_> = tree.keys().cloned().collect();

        for dir in sorted_dirs {
            let files = tree.get(&dir).unwrap();

            // Add directory header if not root
            if !dir.is_empty() {
                lines.push(Line::from(vec![Span::styled(
                    format!("{}/", dir),
                    Style::default()
                        .fg(theme.file_tree_directory)
                        .add_modifier(Modifier::BOLD),
                )]));
            }

            // Add files
            for (index, filename, status_char, color, additions, deletions) in files {
                let is_current = *index == current_file_index;
                let indent = if dir.is_empty() { "" } else { "  " }.to_string();
                let indent_len = indent.len();

                let status_str = format!("{} ", status_char);
                let stats_str = format!(" +{} -{}", additions, deletions);

                let mut spans = vec![];

                if is_current {
                    // Current file - apply background to entire line
                    spans.push(Span::styled(
                        indent.clone(),
                        Style::default().bg(theme.file_tree_current_file_bg),
                    ));
                    spans.push(Span::styled(
                        status_str.clone(),
                        Style::default()
                            .fg(*color)
                            .bg(theme.file_tree_current_file_bg)
                            .add_modifier(Modifier::BOLD),
                    ));
                    spans.push(Span::styled(
                        filename.clone(),
                        Style::default()
                            .fg(theme.file_tree_current_file_fg)
                            .bg(theme.file_tree_current_file_bg)
                            .add_modifier(Modifier::BOLD),
                    ));
                    spans.push(Span::styled(
                        format!(" +{}", additions),
                        Style::default()
                            .fg(theme.file_tree_stats_added)
                            .bg(theme.file_tree_current_file_bg),
                    ));
                    spans.push(Span::styled(
                        format!(" -{}", deletions),
                        Style::default()
                            .fg(theme.file_tree_stats_deleted)
                            .bg(theme.file_tree_current_file_bg),
                    ));

                    // Calculate line length and fill to right edge
                    let line_len = indent_len + status_str.len() + filename.len() + stats_str.len();
                    if line_len < area_width {
                        spans.push(Span::styled(
                            " ".repeat(area_width - line_len),
                            Style::default().bg(theme.file_tree_current_file_bg),
                        ));
                    }
                } else {
                    // Normal file
                    spans.push(Span::raw(indent));
                    spans.push(Span::styled(
                        status_str,
                        Style::default().fg(*color).add_modifier(Modifier::BOLD),
                    ));
                    spans.push(Span::styled(
                        filename.clone(),
                        Style::default().fg(theme.file_tree_default),
                    ));
                    spans.push(Span::styled(
                        format!(" +{}", additions),
                        Style::default().fg(theme.file_tree_stats_added),
                    ));
                    spans.push(Span::styled(
                        format!(" -{}", deletions),
                        Style::default().fg(theme.file_tree_stats_deleted),
                    ));
                }

                lines.push(Line::from(spans));
            }
        }

        lines
    }
}
