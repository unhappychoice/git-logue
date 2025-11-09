use crate::animation::{ActivePane, AnimationEngine};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct EditorPane;

struct HighlightContext<'a> {
    line_content: &'a str,
    line_num: usize,
    show_cursor: bool,
    cursor_col: usize,
    cursor_line: usize,
    old_highlights: &'a [crate::syntax::HighlightSpan],
    new_highlights: &'a [crate::syntax::HighlightSpan],
    old_line_offsets: &'a [usize],
    new_line_offsets: &'a [usize],
    line_offset: isize,
}

impl EditorPane {
    pub fn render(&self, f: &mut Frame, area: Rect, engine: &AnimationEngine) {
        let block = Block::default()
            .title("Editor")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        let content_height = area.height.saturating_sub(2) as usize;
        let scroll_offset = engine.buffer.scroll_offset;
        let buffer_lines = &engine.buffer.lines;
        let line_num_width = format!("{}", buffer_lines.len()).len().max(3);

        let visible_lines: Vec<Line> = buffer_lines
            .iter()
            .skip(scroll_offset)
            .take(content_height)
            .enumerate()
            .map(|(idx, line_content)| {
                let line_num = scroll_offset + idx;
                self.build_line(line_content, line_num, line_num_width, engine)
            })
            .collect();

        let content = Paragraph::new(visible_lines).block(block);
        f.render_widget(content, area);
    }

    fn build_line(
        &self,
        line_content: &str,
        line_num: usize,
        line_num_width: usize,
        engine: &AnimationEngine,
    ) -> Line<'_> {
        let is_cursor_line = line_num == engine.buffer.cursor_line;
        let mut spans = Vec::new();

        spans.push(self.render_line_number(line_num, is_cursor_line, line_num_width));
        spans.push(Span::styled("│ ", Style::default().fg(Color::DarkGray)));

        let line_spans = self.highlight_line(HighlightContext {
            line_content,
            line_num,
            show_cursor: is_cursor_line
                && engine.cursor_visible
                && engine.active_pane == ActivePane::Editor,
            cursor_col: engine.buffer.cursor_col,
            cursor_line: engine.buffer.cursor_line,
            old_highlights: &engine.buffer.old_highlights,
            new_highlights: &engine.buffer.new_highlights,
            old_line_offsets: &engine.buffer.old_content_line_offsets,
            new_line_offsets: &engine.buffer.new_content_line_offsets,
            line_offset: engine.line_offset,
        });
        spans.extend(line_spans);

        Line::from(spans)
    }

    fn render_line_number(&self, line_num: usize, is_cursor_line: bool, width: usize) -> Span<'_> {
        let line_num_str = format!("{:>width$} ", line_num + 1, width = width);
        if is_cursor_line {
            Span::styled(
                line_num_str,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(line_num_str, Style::default().fg(Color::DarkGray))
        }
    }

    fn highlight_line(&self, ctx: HighlightContext<'_>) -> Vec<Span<'_>> {
        let (highlights, line_offsets) = self.select_highlights_and_offsets(
            ctx.line_num,
            ctx.cursor_line,
            ctx.old_highlights,
            ctx.new_highlights,
            ctx.old_line_offsets,
            ctx.new_line_offsets,
        );

        let byte_offset = self.calculate_byte_offset(
            ctx.line_num,
            ctx.cursor_line,
            ctx.line_offset,
            line_offsets,
        );

        let line_highlights =
            self.filter_line_highlights(highlights, byte_offset, ctx.line_content.len());

        self.apply_highlights(
            ctx.line_content,
            byte_offset,
            &line_highlights,
            ctx.show_cursor,
            ctx.cursor_col,
        )
    }

    fn select_highlights_and_offsets<'a>(
        &self,
        line_num: usize,
        cursor_line: usize,
        old_highlights: &'a [crate::syntax::HighlightSpan],
        new_highlights: &'a [crate::syntax::HighlightSpan],
        old_line_offsets: &'a [usize],
        new_line_offsets: &'a [usize],
    ) -> (&'a [crate::syntax::HighlightSpan], &'a [usize]) {
        if line_num < cursor_line {
            (new_highlights, new_line_offsets)
        } else if line_num == cursor_line {
            (&[], &[])
        } else {
            (old_highlights, old_line_offsets)
        }
    }

    fn calculate_byte_offset(
        &self,
        line_num: usize,
        cursor_line: usize,
        line_offset: isize,
        line_offsets: &[usize],
    ) -> usize {
        let target_line = if line_num > cursor_line {
            ((line_num as isize) - line_offset).max(0) as usize
        } else {
            line_num
        };

        line_offsets
            .get(target_line)
            .copied()
            .unwrap_or_else(|| *line_offsets.last().unwrap_or(&0))
    }

    fn filter_line_highlights(
        &self,
        highlights: &[crate::syntax::HighlightSpan],
        byte_offset: usize,
        line_len: usize,
    ) -> Vec<(usize, usize, crate::syntax::TokenType)> {
        let line_end = byte_offset + line_len;
        highlights
            .iter()
            .filter_map(|h| {
                if h.start < line_end && h.end > byte_offset {
                    Some((h.start, h.end, h.token_type))
                } else {
                    None
                }
            })
            .collect()
    }

    fn apply_highlights(
        &self,
        line_content: &str,
        byte_offset: usize,
        line_highlights: &[(usize, usize, crate::syntax::TokenType)],
        show_cursor: bool,
        cursor_col: usize,
    ) -> Vec<Span<'_>> {
        let chars: Vec<char> = line_content.chars().collect();
        let mut spans = Vec::new();

        let mut relative_byte = 0;
        for (char_idx, ch) in chars.iter().enumerate() {
            let char_byte_start = byte_offset + relative_byte;
            let char_byte_end = char_byte_start + ch.len_utf8();
            relative_byte += ch.len_utf8();

            let color = self.get_char_color(char_byte_start, char_byte_end, line_highlights);

            if show_cursor && char_idx == cursor_col {
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
        }

        if show_cursor && cursor_col >= chars.len() {
            spans.push(Span::styled(
                " ",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        spans
    }

    fn get_char_color(
        &self,
        char_byte_start: usize,
        char_byte_end: usize,
        line_highlights: &[(usize, usize, crate::syntax::TokenType)],
    ) -> Color {
        line_highlights
            .iter()
            .find(|h| char_byte_start >= h.0 && char_byte_end <= h.1)
            .map(|h| h.2.color())
            .unwrap_or(Color::White)
    }
}
