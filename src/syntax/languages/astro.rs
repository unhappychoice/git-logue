pub fn language() -> tree_sitter::Language {
    tree_sitter_astro_next::LANGUAGE.into()
}

pub const HIGHLIGHT_QUERY: &str = tree_sitter_astro_next::HIGHLIGHTS_QUERY;
