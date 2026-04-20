pub mod languages;

use crate::theme::Theme;
use ratatui::style::Color;
use std::ops::Range;
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor, QueryMatch, Tree};

pub use languages::{get_language, get_language_by_name, LanguageSupport};

const MAX_INJECTION_DEPTH: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Comment,
    Constant,
    Function,
    Keyword,
    Label,
    Number,
    Operator,
    Parameter,
    Property,
    Punctuation,
    String,
    Type,
    Variable,
}

impl TokenType {
    pub fn color(&self, theme: &Theme) -> Color {
        match self {
            TokenType::Comment => theme.syntax_comment,
            TokenType::Constant => theme.syntax_constant,
            TokenType::Function => theme.syntax_function,
            TokenType::Keyword => theme.syntax_keyword,
            TokenType::Label => theme.syntax_label,
            TokenType::Number => theme.syntax_number,
            TokenType::Operator => theme.syntax_operator,
            TokenType::Parameter => theme.syntax_parameter,
            TokenType::Property => theme.syntax_property,
            TokenType::Punctuation => theme.syntax_punctuation,
            TokenType::String => theme.syntax_string,
            TokenType::Type => theme.syntax_type,
            TokenType::Variable => theme.syntax_variable,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
}

pub struct Highlighter {
    parser: Parser,
    language: Option<Language>,
    highlight_query: Option<Query>,
    highlight_query_source: Option<String>,
    injection_query: Option<Query>,
    injection_query_source: Option<String>,
    cached_tree: Option<Tree>,
    cached_source: String,
}

impl Clone for Highlighter {
    fn clone(&self) -> Self {
        let mut parser = Parser::new();
        let (highlight_query, injection_query) = self
            .language
            .as_ref()
            .map(|lang| {
                let _ = parser.set_language(lang);
                let hq = self
                    .highlight_query_source
                    .as_ref()
                    .and_then(|src| Query::new(lang, src).ok());
                let iq = self
                    .injection_query_source
                    .as_ref()
                    .and_then(|src| Query::new(lang, src).ok());
                (hq, iq)
            })
            .unwrap_or((None, None));

        Self {
            parser,
            language: self.language.clone(),
            highlight_query,
            highlight_query_source: self.highlight_query_source.clone(),
            injection_query,
            injection_query_source: self.injection_query_source.clone(),
            cached_tree: None,
            cached_source: String::new(),
        }
    }
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            language: None,
            highlight_query: None,
            highlight_query_source: None,
            injection_query: None,
            injection_query_source: None,
            cached_tree: None,
            cached_source: String::new(),
        }
    }

    pub fn set_language_from_path(&mut self, path: &str) -> bool {
        self.clear_language();
        let Some(support) = get_language(Path::new(path)) else {
            return false;
        };
        if self.parser.set_language(&support.language).is_err() {
            return false;
        }
        let Ok(highlight_query) = Query::new(&support.language, support.highlight_query) else {
            return false;
        };
        self.highlight_query = Some(highlight_query);
        self.highlight_query_source = Some(support.highlight_query.to_string());
        if let Some(src) = support.injection_query {
            if let Ok(query) = Query::new(&support.language, src) {
                self.injection_query = Some(query);
                self.injection_query_source = Some(src.to_string());
            }
        }
        self.language = Some(support.language);
        true
    }

    fn clear_language(&mut self) {
        self.language = None;
        self.highlight_query = None;
        self.highlight_query_source = None;
        self.injection_query = None;
        self.injection_query_source = None;
        self.cached_tree = None;
        self.cached_source.clear();
    }

    pub fn highlight(&mut self, source: &str) -> Vec<HighlightSpan> {
        let Some(highlight_query) = &self.highlight_query else {
            return Vec::new();
        };

        let old_tree = if self.cached_source == source {
            self.cached_tree.as_ref()
        } else {
            None
        };
        let Some(tree) = self.parser.parse(source, old_tree) else {
            return Vec::new();
        };
        self.cached_tree = Some(tree.clone());
        self.cached_source = source.to_string();

        let outer = collect_spans(highlight_query, &tree, source);
        let injections = self
            .injection_query
            .as_ref()
            .map(|q| gather_injections(q, &tree, source, 0))
            .unwrap_or_default();
        merge_injection_spans(outer, injections)
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

fn collect_spans(query: &Query, tree: &Tree, source: &str) -> Vec<HighlightSpan> {
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    let mut spans = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let Some(token_type) = capture_name_to_token(capture_name) else {
                continue;
            };
            spans.push(HighlightSpan {
                start: capture.node.start_byte(),
                end: capture.node.end_byte(),
                token_type,
            });
        }
    }
    spans.sort_by_key(|span| span.start);
    spans
}

fn capture_name_to_token(capture_name: &str) -> Option<TokenType> {
    let base = capture_name.split('.').next().unwrap_or(capture_name);
    let token = match base {
        "annotation" | "attribute" | "decorator" => TokenType::Keyword,
        "boolean" => TokenType::Constant,
        "character" => TokenType::String,
        "class" | "constructor" | "enum" | "interface" | "struct" | "trait" => TokenType::Type,
        "comment" => TokenType::Comment,
        "conditional" | "exception" | "include" | "repeat" | "storageclass" => TokenType::Keyword,
        "constant" => TokenType::Constant,
        "delimiter" => TokenType::Punctuation,
        "escape" => TokenType::Operator,
        "field" => TokenType::Property,
        "float" => TokenType::Number,
        "function" => TokenType::Function,
        "identifier" => TokenType::Variable,
        "keyword" => TokenType::Keyword,
        "label" => TokenType::Label,
        "macro" | "method" => TokenType::Function,
        "module" | "namespace" => TokenType::Type,
        "number" => TokenType::Number,
        "operator" => TokenType::Operator,
        "parameter" => TokenType::Parameter,
        "property" => TokenType::Property,
        "punctuation" => TokenType::Punctuation,
        "regexp" => TokenType::String,
        "special" => TokenType::Operator,
        "string" => TokenType::String,
        "tag" => TokenType::Type,
        "text" => TokenType::String,
        "type" => TokenType::Type,
        "variable" => TokenType::Variable,
        _ => return None,
    };
    Some(token)
}

fn gather_injections(
    query: &Query,
    tree: &Tree,
    source: &str,
    depth: u8,
) -> Vec<(Range<usize>, Vec<HighlightSpan>)> {
    if depth >= MAX_INJECTION_DEPTH {
        return Vec::new();
    }
    let content_index = capture_index(query, "injection.content");
    let Some(content_index) = content_index else {
        return Vec::new();
    };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    let mut out = Vec::new();
    while let Some(m) = matches.next() {
        let Some(lang_name) = resolve_injection_language(query, m, source) else {
            continue;
        };
        let Some(support) = get_language_by_name(&lang_name) else {
            continue;
        };
        for capture in m.captures {
            if capture.index != content_index {
                continue;
            }
            let range = capture.node.byte_range();
            if range.start >= range.end || range.end > source.len() {
                continue;
            }
            let slice = &source[range.clone()];
            let inner = highlight_slice(&support, slice, range.start, depth + 1);
            out.push((range, inner));
        }
    }
    out
}

fn capture_index(query: &Query, name: &str) -> Option<u32> {
    query
        .capture_names()
        .iter()
        .position(|n| *n == name)
        .map(|i| i as u32)
}

fn resolve_injection_language(query: &Query, m: &QueryMatch, source: &str) -> Option<String> {
    for prop in query.property_settings(m.pattern_index) {
        if prop.key.as_ref() == "injection.language" {
            if let Some(value) = &prop.value {
                return Some(value.to_ascii_lowercase());
            }
        }
    }
    let lang_idx = capture_index(query, "injection.language")?;
    m.captures
        .iter()
        .find(|c| c.index == lang_idx)
        .and_then(|c| {
            let text = source.get(c.node.byte_range())?.trim();
            let cleaned = text.trim_matches(|ch: char| ch == '"' || ch == '\'' || ch == '`');
            let first = cleaned
                .split(|ch: char| ch.is_whitespace() || ch == ',' || ch == '{')
                .next()?
                .trim();
            (!first.is_empty()).then(|| first.to_ascii_lowercase())
        })
}

fn highlight_slice(
    support: &LanguageSupport,
    source: &str,
    base_offset: usize,
    depth: u8,
) -> Vec<HighlightSpan> {
    let mut parser = Parser::new();
    if parser.set_language(&support.language).is_err() {
        return Vec::new();
    }
    let Ok(highlight_query) = Query::new(&support.language, support.highlight_query) else {
        return Vec::new();
    };
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };

    let outer = collect_spans(&highlight_query, &tree, source);
    let injections = support
        .injection_query
        .and_then(|src| Query::new(&support.language, src).ok())
        .map(|q| gather_injections(&q, &tree, source, depth))
        .unwrap_or_default();
    let merged = merge_injection_spans(outer, injections);
    merged
        .into_iter()
        .map(|s| HighlightSpan {
            start: s.start + base_offset,
            end: s.end + base_offset,
            token_type: s.token_type,
        })
        .collect()
}

fn merge_injection_spans(
    outer: Vec<HighlightSpan>,
    injections: Vec<(Range<usize>, Vec<HighlightSpan>)>,
) -> Vec<HighlightSpan> {
    if injections.is_empty() {
        return outer;
    }
    let ranges: Vec<Range<usize>> = injections.iter().map(|(r, _)| r.clone()).collect();
    let mut merged: Vec<HighlightSpan> = outer
        .into_iter()
        .filter(|s| !ranges.iter().any(|r| s.start < r.end && r.start < s.end))
        .collect();
    for (_, inner) in injections {
        merged.extend(inner);
    }
    merged.sort_by_key(|span| span.start);
    merged
}
