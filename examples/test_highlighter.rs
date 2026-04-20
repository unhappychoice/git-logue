fn main() {
    // Test Rust highlighting
    let test_code = r#"fn main() {
    let x = 42;
    println!("Hello, world!");
}
"#;

    println!("=== Testing Rust highlighting ===");
    let mut highlighter = gitlogue::syntax::Highlighter::new();
    let success = highlighter.set_language_from_path("test.rs");
    println!("Language set: {}", success);

    let highlights = highlighter.highlight(test_code);
    println!("Number of highlights: {}", highlights.len());

    for (i, span) in highlights.iter().enumerate().take(10) {
        let text = &test_code[span.start..span.end];
        println!(
            "{}: [{}-{}] {:?} = '{}'",
            i, span.start, span.end, span.token_type, text
        );
    }

    // Test Markdown highlighting
    let markdown_code = r#"# Hello World

This is a **bold** text and *italic* text.

```rust
fn main() {
    println!("Hello");
}
```

- List item 1
- List item 2

[Link](https://example.com)
"#;

    println!("\n=== Testing Markdown highlighting ===");
    let mut md_highlighter = gitlogue::syntax::Highlighter::new();
    let md_success = md_highlighter.set_language_from_path("test.md");
    println!("Language set: {}", md_success);

    let md_highlights = md_highlighter.highlight(markdown_code);
    println!("Number of highlights: {}", md_highlights.len());

    for (i, span) in md_highlights.iter().enumerate().take(20) {
        let text = &markdown_code[span.start..span.end];
        println!(
            "{}: [{}-{}] {:?} = '{}'",
            i, span.start, span.end, span.token_type, text
        );
    }

    let astro_code = r#"---
const name = "world";
---
<h1>Hello {name}</h1>
<style>
  h1 { color: red; }
</style>
"#;
    println!("\n=== Testing Astro highlighting ===");
    let mut astro_highlighter = gitlogue::syntax::Highlighter::new();
    let astro_success = astro_highlighter.set_language_from_path("test.astro");
    println!("Language set: {}", astro_success);
    let astro_highlights = astro_highlighter.highlight(astro_code);
    println!("Number of highlights: {}", astro_highlights.len());
    for (i, span) in astro_highlights.iter().enumerate().take(25) {
        let text = &astro_code[span.start..span.end];
        println!(
            "{}: [{}-{}] {:?} = '{}'",
            i, span.start, span.end, span.token_type, text
        );
    }

    let html_code = r#"<!DOCTYPE html>
<html>
<head><style>body { color: blue; }</style></head>
<body><script>const x = 1;</script></body>
</html>
"#;
    println!("\n=== Testing HTML highlighting ===");
    let mut html_highlighter = gitlogue::syntax::Highlighter::new();
    let html_success = html_highlighter.set_language_from_path("test.html");
    println!("Language set: {}", html_success);
    let html_highlights = html_highlighter.highlight(html_code);
    println!("Number of highlights: {}", html_highlights.len());
    for (i, span) in html_highlights.iter().enumerate().take(25) {
        let text = &html_code[span.start..span.end];
        println!(
            "{}: [{}-{}] {:?} = '{}'",
            i, span.start, span.end, span.token_type, text
        );
    }
}
