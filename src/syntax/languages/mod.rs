pub mod astro;
pub mod bash;
pub mod c;
pub mod clojure;
pub mod cpp;
pub mod csharp;
pub mod css;
pub mod dart;
pub mod elixir;
pub mod erlang;
pub mod go_lang;
pub mod haskell;
pub mod html;
pub mod java;
pub mod javascript;
pub mod json;
pub mod kotlin;
pub mod lua;
pub mod markdown;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod svelte;
pub mod swift;
pub mod typescript;
pub mod xml;
pub mod yaml;
pub mod zig;

use std::path::Path;
use tree_sitter::Language;

#[derive(Clone)]
pub struct LanguageSupport {
    pub language: Language,
    pub highlight_query: &'static str,
    pub injection_query: Option<&'static str>,
}

pub fn get_language(path: &Path) -> Option<LanguageSupport> {
    let extension = path.extension()?.to_str()?;
    by_name(extension)
}

pub fn get_language_by_name(name: &str) -> Option<LanguageSupport> {
    by_name(name.trim().trim_start_matches('.'))
}

fn by_name(raw: &str) -> Option<LanguageSupport> {
    let canonical = canonicalize(raw)?;
    match canonical {
        "astro" => Some(LanguageSupport {
            language: astro::language(),
            highlight_query: astro::HIGHLIGHT_QUERY,
            injection_query: Some(astro::INJECTION_QUERY),
        }),
        "bash" => Some(LanguageSupport {
            language: bash::language(),
            highlight_query: bash::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "c" => Some(LanguageSupport {
            language: c::language(),
            highlight_query: c::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "clojure" => Some(LanguageSupport {
            language: clojure::language(),
            highlight_query: clojure::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "cpp" => Some(LanguageSupport {
            language: cpp::language(),
            highlight_query: cpp::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "csharp" => Some(LanguageSupport {
            language: csharp::language(),
            highlight_query: csharp::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "css" => Some(LanguageSupport {
            language: css::language(),
            highlight_query: css::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "dart" => Some(LanguageSupport {
            language: dart::language(),
            highlight_query: dart::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "elixir" => Some(LanguageSupport {
            language: elixir::language(),
            highlight_query: elixir::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "erlang" => Some(LanguageSupport {
            language: erlang::language(),
            highlight_query: erlang::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "go" => Some(LanguageSupport {
            language: go_lang::language(),
            highlight_query: go_lang::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "haskell" => Some(LanguageSupport {
            language: haskell::language(),
            highlight_query: haskell::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "html" => Some(LanguageSupport {
            language: html::language(),
            highlight_query: html::HIGHLIGHT_QUERY,
            injection_query: Some(html::INJECTION_QUERY),
        }),
        "java" => Some(LanguageSupport {
            language: java::language(),
            highlight_query: java::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "javascript" => Some(LanguageSupport {
            language: javascript::language(),
            highlight_query: javascript::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "json" => Some(LanguageSupport {
            language: json::language(),
            highlight_query: json::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "kotlin" => Some(LanguageSupport {
            language: kotlin::language(),
            highlight_query: kotlin::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "lua" => Some(LanguageSupport {
            language: lua::language(),
            highlight_query: lua::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "markdown" => Some(LanguageSupport {
            language: markdown::language(),
            highlight_query: markdown::HIGHLIGHT_QUERY,
            injection_query: Some(markdown::INJECTION_QUERY),
        }),
        "php" => Some(LanguageSupport {
            language: php::language(),
            highlight_query: php::HIGHLIGHT_QUERY,
            injection_query: Some(php::INJECTION_QUERY),
        }),
        "python" => Some(LanguageSupport {
            language: python::language(),
            highlight_query: python::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "ruby" => Some(LanguageSupport {
            language: ruby::language(),
            highlight_query: ruby::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "rust" => Some(LanguageSupport {
            language: rust::language(),
            highlight_query: rust::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "scala" => Some(LanguageSupport {
            language: scala::language(),
            highlight_query: scala::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "svelte" => Some(LanguageSupport {
            language: svelte::language(),
            highlight_query: svelte::HIGHLIGHT_QUERY,
            injection_query: Some(svelte::INJECTION_QUERY),
        }),
        "swift" => Some(LanguageSupport {
            language: swift::language(),
            highlight_query: swift::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "typescript" => Some(LanguageSupport {
            language: typescript::language(),
            highlight_query: typescript::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "xml" => Some(LanguageSupport {
            language: xml::language(),
            highlight_query: xml::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "yaml" => Some(LanguageSupport {
            language: yaml::language(),
            highlight_query: yaml::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        "zig" => Some(LanguageSupport {
            language: zig::language(),
            highlight_query: zig::HIGHLIGHT_QUERY,
            injection_query: None,
        }),
        _ => None,
    }
}

fn canonicalize(raw: &str) -> Option<&'static str> {
    let lower = raw.to_ascii_lowercase();
    match lower.as_str() {
        "astro" => Some("astro"),
        "sh" | "bash" | "zsh" | "shell" => Some("bash"),
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "h++" | "tcc" | "inl" => Some("cpp"),
        "c" | "h" => Some("c"),
        "clj" | "cljs" | "cljc" | "edn" | "clojure" => Some("clojure"),
        "cs" | "csx" | "csharp" | "c#" => Some("csharp"),
        "css" | "scss" | "postcss" | "less" => Some("css"),
        "dart" => Some("dart"),
        "ex" | "exs" | "elixir" => Some("elixir"),
        "erl" | "hrl" | "es" | "escript" | "erlang" => Some("erlang"),
        "go" | "golang" => Some("go"),
        "hs" | "lhs" | "haskell" => Some("haskell"),
        "html" | "htm" => Some("html"),
        "java" => Some("java"),
        "js" | "jsx" | "mjs" | "cjs" | "javascript" => Some("javascript"),
        "json" | "jsonc" | "json5" => Some("json"),
        "kt" | "kts" | "kotlin" => Some("kotlin"),
        "lua" => Some("lua"),
        "md" | "markdown" => Some("markdown"),
        "php" | "php3" | "php4" | "php5" | "phtml" => Some("php"),
        "py" | "pyw" | "python" => Some("python"),
        "rb" | "rbw" | "rake" | "gemspec" | "ruby" => Some("ruby"),
        "rs" | "rust" => Some("rust"),
        "scala" | "sc" | "sbt" => Some("scala"),
        "svelte" => Some("svelte"),
        "swift" => Some("swift"),
        "ts" | "tsx" | "mts" | "cts" | "typescript" => Some("typescript"),
        "xml" | "svg" | "xsl" | "xslt" => Some("xml"),
        "yaml" | "yml" => Some("yaml"),
        "zig" => Some("zig"),
        _ => None,
    }
}
