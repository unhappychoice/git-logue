pub fn language() -> tree_sitter::Language {
    tree_sitter_svelte_ng::LANGUAGE.into()
}

pub const HIGHLIGHT_QUERY: &str = tree_sitter_svelte_ng::HIGHLIGHTS_QUERY;

pub const INJECTION_QUERY: &str = tree_sitter_svelte_ng::INJECTIONS_QUERY;
