use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    // Background colors
    pub background_left: Color,  // FileTree and StatusBar side (darker)
    pub background_right: Color, // Editor and Terminal side

    // Editor colors
    pub editor_line_number: Color,
    pub editor_line_number_cursor: Color,
    pub editor_separator: Color,
    pub editor_cursor_char_bg: Color,
    pub editor_cursor_char_fg: Color,
    pub editor_cursor_line_bg: Color,

    // File tree colors
    pub file_tree_added: Color,
    pub file_tree_deleted: Color,
    pub file_tree_modified: Color,
    pub file_tree_renamed: Color,
    pub file_tree_directory: Color,
    pub file_tree_current_file_bg: Color,
    pub file_tree_current_file_fg: Color,
    pub file_tree_default: Color,
    pub file_tree_stats_added: Color,
    pub file_tree_stats_deleted: Color,

    // Terminal colors
    pub terminal_command: Color,
    pub terminal_output: Color,
    pub terminal_cursor_bg: Color,
    pub terminal_cursor_fg: Color,

    // Status bar colors
    pub status_hash: Color,
    pub status_author: Color,
    pub status_date: Color,
    pub status_message: Color,
    pub status_no_commit: Color,

    // Separator colors
    pub separator: Color,

    // Syntax highlighting colors
    pub syntax_keyword: Color,
    pub syntax_type: Color,
    pub syntax_function: Color,
    pub syntax_variable: Color,
    pub syntax_string: Color,
    pub syntax_number: Color,
    pub syntax_comment: Color,
    pub syntax_operator: Color,
    pub syntax_punctuation: Color,
    pub syntax_constant: Color,
    pub syntax_parameter: Color,
    pub syntax_property: Color,
    pub syntax_label: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::tokyo_night()
    }
}

impl Theme {
    /// Tokyo Night inspired color scheme
    pub fn tokyo_night() -> Self {
        Self {
            // Background colors
            background_left: Color::Rgb(30, 34, 54), // Darker for left side
            background_right: Color::Rgb(26, 27, 38), // Base background

            // Editor colors
            editor_line_number: Color::Rgb(86, 95, 137),
            editor_line_number_cursor: Color::Rgb(125, 207, 255), // Cyan
            editor_separator: Color::Rgb(86, 95, 137),
            editor_cursor_char_bg: Color::Rgb(122, 162, 247),
            editor_cursor_char_fg: Color::Rgb(26, 27, 38),
            editor_cursor_line_bg: Color::Rgb(42, 47, 68),

            // File tree colors
            file_tree_added: Color::Rgb(158, 206, 106), // Green
            file_tree_deleted: Color::Rgb(247, 118, 142), // Red
            file_tree_modified: Color::Rgb(255, 158, 100), // Orange
            file_tree_renamed: Color::Rgb(122, 162, 247), // Blue
            file_tree_directory: Color::Rgb(122, 162, 247), // Blue
            file_tree_current_file_bg: Color::Rgb(42, 47, 68),
            file_tree_current_file_fg: Color::Rgb(192, 202, 245),
            file_tree_default: Color::Rgb(192, 202, 245),
            file_tree_stats_added: Color::Rgb(158, 206, 106), // Green
            file_tree_stats_deleted: Color::Rgb(247, 118, 142), // Red

            // Terminal colors
            terminal_command: Color::Rgb(192, 202, 245), // Light foreground
            terminal_output: Color::Rgb(86, 95, 137),    // Muted gray (less prominent)
            terminal_cursor_bg: Color::Rgb(122, 162, 247),
            terminal_cursor_fg: Color::Rgb(26, 27, 38),

            // Status bar colors
            status_hash: Color::Rgb(255, 213, 128),   // Yellow
            status_author: Color::Rgb(158, 206, 106), // Green
            status_date: Color::Rgb(122, 162, 247),   // Blue
            status_message: Color::Rgb(192, 202, 245),
            status_no_commit: Color::Rgb(86, 95, 137), // Muted gray

            // Separator colors
            separator: Color::Rgb(86, 95, 137),

            // Syntax highlighting colors (Tokyo Night inspired)
            syntax_keyword: Color::Rgb(187, 154, 247), // Purple
            syntax_type: Color::Rgb(125, 207, 255),    // Cyan
            syntax_function: Color::Rgb(122, 162, 247), // Blue
            syntax_variable: Color::Rgb(192, 202, 245), // Light foreground
            syntax_string: Color::Rgb(158, 206, 106),  // Green
            syntax_number: Color::Rgb(255, 158, 100),  // Orange
            syntax_comment: Color::Rgb(86, 95, 137),   // Muted blue-gray
            syntax_operator: Color::Rgb(125, 207, 255), // Cyan
            syntax_punctuation: Color::Rgb(140, 148, 184), // Light gray-blue
            syntax_constant: Color::Rgb(255, 158, 100), // Orange
            syntax_parameter: Color::Rgb(255, 213, 128), // Yellow
            syntax_property: Color::Rgb(158, 206, 106), // Green
            syntax_label: Color::Rgb(187, 154, 247),   // Purple
        }
    }
}
