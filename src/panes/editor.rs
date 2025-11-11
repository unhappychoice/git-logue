use crate::animation::{ActivePane, AnimationEngine};
use crate::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub struct EditorPane;

struct HighlightContext<'a> {
    line_content: &'a str,
    line_num: usize,
    is_cursor_line: bool,
    show_cursor: bool,
    cursor_col: usize,
    cursor_line: usize,
    old_highlights: &'a [crate::syntax::HighlightSpan],
    new_highlights: &'a [crate::syntax::HighlightSpan],
    old_line_offsets: &'a [usize],
    new_line_offsets: &'a [usize],
    line_offset: isize,
    theme: &'a Theme,
    distance_opacity: f32,
}

impl EditorPane {
    pub fn render(&self, f: &mut Frame, area: Rect, engine: &AnimationEngine, theme: &Theme) {
        let block = Block::default()
            .style(Style::default().bg(theme.background_right))
            .padding(Padding::vertical(1));

        let content_height = area.height.saturating_sub(2) as usize; // Subtract top and bottom padding
                                                                     // Note: Padding::vertical doesn't affect width, so content_width = area.width
        let content_width = area.width as usize;
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
                self.build_line(
                    line_content,
                    line_num,
                    line_num_width,
                    content_width,
                    engine,
                    theme,
                )
            })
            .collect();

        let content = Paragraph::new(visible_lines)
            .block(block)
            .wrap(Wrap { trim: false });
        f.render_widget(content, area);
    }

    fn build_line(
        &self,
        line_content: &str,
        line_num: usize,
        line_num_width: usize,
        content_width: usize,
        engine: &AnimationEngine,
        theme: &Theme,
    ) -> Line<'_> {
        let is_cursor_line = line_num == engine.buffer.cursor_line;
        let cursor_line = engine.buffer.cursor_line;

        // Calculate distance-based opacity (closer to cursor = brighter)
        let distance = (line_num as isize - cursor_line as isize).unsigned_abs();
        let max_distance = 20; // Lines beyond this distance have minimum opacity
        let distance_opacity = if distance == 0 {
            1.0 // Cursor line is full brightness
        } else {
            // Gradually fade from 1.0 to 0.6 based on distance
            1.0 - (distance.min(max_distance) as f32 / max_distance as f32) * 0.5
        };

        let mut spans = Vec::new();

        // Left padding
        if is_cursor_line {
            spans.push(Span::styled(
                "  ",
                Style::default().bg(theme.editor_cursor_line_bg),
            ));
        } else {
            spans.push(Span::raw("  "));
        }

        spans.push(self.render_line_number(
            line_num,
            is_cursor_line,
            line_num_width,
            theme,
            distance_opacity,
        ));

        let separator_style = if is_cursor_line {
            Style::default()
                .fg(theme.editor_separator)
                .bg(theme.editor_cursor_line_bg)
        } else {
            let separator_color = self.apply_opacity(
                theme.editor_separator,
                distance_opacity,
                theme.background_right,
            );
            Style::default().fg(separator_color)
        };
        spans.push(Span::styled("  ", separator_style));

        let show_cursor =
            is_cursor_line && engine.cursor_visible && engine.active_pane == ActivePane::Editor;

        let line_spans = self.highlight_line(HighlightContext {
            line_content,
            line_num,
            is_cursor_line,
            show_cursor,
            cursor_col: engine.buffer.cursor_col,
            cursor_line: engine.buffer.cursor_line,
            old_highlights: &engine.buffer.old_highlights,
            new_highlights: &engine.buffer.new_highlights,
            old_line_offsets: &engine.buffer.old_content_line_offsets,
            new_line_offsets: &engine.buffer.new_content_line_offsets,
            line_offset: engine.line_offset,
            theme,
            distance_opacity,
        });

        spans.extend(line_spans);

        // Right padding
        if is_cursor_line {
            spans.push(Span::styled(
                "  ",
                Style::default().bg(theme.editor_cursor_line_bg),
            ));
        } else {
            spans.push(Span::raw("  "));
        }

        // Fill cursor line to the right edge with background color
        if is_cursor_line {
            // Calculate total display width already added to spans
            // Use unicode width instead of char count to handle wide characters (CJK, emojis, etc.)
            let current_width: usize = spans.iter().map(|s| s.content.width()).sum();

            // Fill remaining space to content_width
            if current_width < content_width {
                let fill_count = content_width - current_width;
                spans.push(Span::styled(
                    " ".repeat(fill_count),
                    Style::default().bg(theme.editor_cursor_line_bg),
                ));
            }
        }

        Line::from(spans)
    }

    fn render_line_number(
        &self,
        line_num: usize,
        is_cursor_line: bool,
        width: usize,
        theme: &Theme,
        distance_opacity: f32,
    ) -> Span<'_> {
        let line_num_str = format!("{:>width$} ", line_num + 1, width = width);

        if is_cursor_line {
            Span::styled(
                line_num_str,
                Style::default()
                    .fg(theme.editor_line_number_cursor)
                    .bg(theme.editor_cursor_line_bg)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            let color = self.apply_opacity(
                theme.editor_line_number,
                distance_opacity,
                theme.background_right,
            );
            Span::styled(line_num_str, Style::default().fg(color))
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

        self.apply_highlights(&line_highlights, byte_offset, &ctx)
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
        if line_num <= cursor_line {
            (new_highlights, new_line_offsets)
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
        line_highlights: &[(usize, usize, crate::syntax::TokenType)],
        byte_offset: usize,
        ctx: &HighlightContext,
    ) -> Vec<Span<'_>> {
        let chars: Vec<char> = ctx.line_content.chars().collect();
        let mut spans = Vec::new();

        let mut relative_byte = 0;
        for (char_idx, ch) in chars.iter().enumerate() {
            let char_byte_start = byte_offset + relative_byte;
            let char_byte_end = char_byte_start + ch.len_utf8();
            relative_byte += ch.len_utf8();

            let mut color =
                self.get_char_color(char_byte_start, char_byte_end, line_highlights, ctx.theme);

            // Apply distance-based opacity (blend with appropriate background)
            let bg_color = if ctx.is_cursor_line {
                ctx.theme.editor_cursor_line_bg
            } else {
                ctx.theme.background_right
            };
            color = self.apply_opacity(color, ctx.distance_opacity, bg_color);

            if ctx.show_cursor && char_idx == ctx.cursor_col {
                // Cursor character - bright highlight
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default()
                        .bg(ctx.theme.editor_cursor_char_bg)
                        .fg(ctx.theme.editor_cursor_char_fg)
                        .add_modifier(Modifier::BOLD),
                ));
            } else if ctx.is_cursor_line {
                // Cursor line - subtle background
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default()
                        .fg(color)
                        .bg(ctx.theme.editor_cursor_line_bg),
                ));
            } else {
                // Normal line
                spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
        }

        if ctx.show_cursor && ctx.cursor_col >= chars.len() {
            spans.push(Span::styled(
                " ",
                Style::default()
                    .bg(ctx.theme.editor_cursor_char_bg)
                    .fg(ctx.theme.editor_cursor_char_fg)
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
        theme: &Theme,
    ) -> Color {
        line_highlights
            .iter()
            .find(|h| char_byte_start >= h.0 && char_byte_end <= h.1)
            .map(|h| h.2.color(theme))
            .unwrap_or(theme.syntax_variable) // Use theme color instead of Color::White
    }

    fn apply_opacity(&self, foreground: Color, opacity: f32, background: Color) -> Color {
        match (foreground, background) {
            (Color::Rgb(fr, fg, fb), Color::Rgb(br, bg, bb)) => {
                // Blend foreground and background: result = fg * opacity + bg * (1 - opacity)
                let r = (fr as f32 * opacity + br as f32 * (1.0 - opacity)) as u8;
                let g = (fg as f32 * opacity + bg as f32 * (1.0 - opacity)) as u8;
                let b = (fb as f32 * opacity + bb as f32 * (1.0 - opacity)) as u8;
                Color::Rgb(r, g, b)
            }
            _ => foreground, // For non-RGB colors, return as-is
        }
    }
}
