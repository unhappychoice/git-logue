use crate::animation::{ActivePane, AnimationEngine};
use crate::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};

pub struct TerminalPane;

impl TerminalPane {
    pub fn render(&self, f: &mut Frame, area: Rect, engine: &AnimationEngine, theme: &Theme) {
        let block = Block::default()
            .style(Style::default().bg(theme.background_right))
            .padding(Padding {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            });

        // Get visible lines based on area height (subtract padding)
        let content_height = area.height.saturating_sub(2) as usize; // Subtract top and bottom padding
        let total_lines = engine.terminal_lines.len();

        let lines: Vec<Line> = if total_lines > 0 {
            let start_idx = total_lines.saturating_sub(content_height);
            engine.terminal_lines[start_idx..]
                .iter()
                .enumerate()
                .map(|(idx, line)| {
                    let is_last_line = start_idx + idx == total_lines - 1;
                    let show_cursor = is_last_line
                        && engine.cursor_visible
                        && engine.active_pane == ActivePane::Terminal;

                    if line.starts_with("~ ") {
                        // Command line
                        if show_cursor {
                            // Add cursor at the end of the line
                            let mut spans = vec![Span::styled(
                                line.clone(),
                                Style::default().fg(theme.terminal_command),
                            )];
                            spans.push(Span::styled(
                                " ",
                                Style::default()
                                    .bg(theme.terminal_cursor_bg)
                                    .fg(theme.terminal_cursor_fg)
                                    .add_modifier(Modifier::BOLD),
                            ));
                            Line::from(spans)
                        } else {
                            Line::from(vec![Span::styled(
                                line.clone(),
                                Style::default().fg(theme.terminal_command),
                            )])
                        }
                    } else {
                        // Output line - normal style
                        Line::from(vec![Span::styled(
                            line.clone(),
                            Style::default().fg(theme.terminal_output),
                        )])
                    }
                })
                .collect()
        } else {
            vec![Line::from("")]
        };

        let content = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });
        f.render_widget(content, area);
    }
}
