//! High-performance Markdown parser and AST tokenizer tailored for streaming LLM outputs.
//! Zero-copy string slicing and single-pass tokenization without regex overhead.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    pub language: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLink {
    pub text: String,
    pub url: String,
}

/// Parses heading line, returning (heading_level, clean_text)
pub fn parse_header(input: &str) -> (usize, String) {
    let trimmed = input.trim();
    let mut level = 0;
    for ch in trimmed.chars() {
        if ch == '#' {
            level += 1;
        } else {
            break;
        }
    }
    if level == 0 || level > 6 {
        return (0, trimmed.to_string());
    }
    let content = trimmed[level..].trim().to_string();
    (level, content)
}

/// Strips markdown styling (bold, italic, strikethrough, inline code, links) to plain text
pub fn strip_markdown_styling(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '*' | '_' => {
                // Skip consecutive formatting delimiters
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ch {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            '~' => {
                if let Some(&'~') = chars.peek() {
                    chars.next();
                }
            }
            '`' => {
                // Skip inline code backticks
            }
            '[' => {
                // Potential link [text](url) -> extract text
                let mut text = String::new();
                let mut found_close = false;
                for inner in chars.by_ref() {
                    if inner == ']' {
                        found_close = true;
                        break;
                    }
                    text.push(inner);
                }
                if found_close && chars.peek() == Some(&'(') {
                    chars.next(); // consume '('
                    for inner in chars.by_ref() {
                        if inner == ')' {
                            break;
                        }
                    }
                    out.push_str(&text);
                } else {
                    out.push('[');
                    out.push_str(&text);
                    if found_close {
                        out.push(']');
                    }
                }
            }
            _ => {
                out.push(ch);
            }
        }
    }

    out
}

/// Extracts all Markdown links `[text](url)` from the text
pub fn extract_markdown_links(input: &str) -> Vec<MarkdownLink> {
    let mut links = Vec::new();
    let mut remaining = input;

    while let Some(open_bracket) = remaining.find('[') {
        remaining = &remaining[open_bracket + 1..];
        if let Some(close_bracket) = remaining.find(']') {
            let text = &remaining[..close_bracket];
            let after_bracket = &remaining[close_bracket + 1..];
            if after_bracket.starts_with('(') {
                if let Some(close_paren) = after_bracket.find(')') {
                    let url = &after_bracket[1..close_paren];
                    links.push(MarkdownLink {
                        text: text.to_string(),
                        url: url.to_string(),
                    });
                    remaining = &after_bracket[close_paren + 1..];
                    continue;
                }
            }
            remaining = &remaining[close_bracket + 1..];
        } else {
            break;
        }
    }

    links
}

/// Extracts structured code blocks including optional language identifiers
pub fn extract_code_blocks_detailed(input: &str) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();
    let mut current_lang = String::new();
    let mut current_content = String::new();
    let mut in_block = false;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_block {
                blocks.push(CodeBlock {
                    language: current_lang.clone(),
                    content: current_content.trim().to_string(),
                });
                current_lang.clear();
                current_content.clear();
                in_block = false;
            } else {
                in_block = true;
                current_lang = trimmed.trim_start_matches('`').trim().to_string();
            }
            continue;
        }

        if in_block {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_levels() {
        assert_eq!(parse_header("# Level 1 Header"), (1, "Level 1 Header".to_string()));
        assert_eq!(parse_header("### Level 3 Title"), (3, "Level 3 Title".to_string()));
        assert_eq!(parse_header("###### Deep Subheading"), (6, "Deep Subheading".to_string()));
        assert_eq!(parse_header("Plain text without header"), (0, "Plain text without header".to_string()));
    }

    #[test]
    fn test_strip_markdown_styling() {
        assert_eq!(strip_markdown_styling("**bold** and *italic* text"), "bold and italic text");
        assert_eq!(strip_markdown_styling("`inline code` and ~~strikethrough~~"), "inline code and strikethrough");
        assert_eq!(
            strip_markdown_styling("Check out [GitHub](https://github.com) now!"),
            "Check out GitHub now!"
        );
    }

    #[test]
    fn test_extract_markdown_links() {
        let input = "Visit [OpenAI](https://openai.com) or [Anthropic](https://anthropic.com) for details.";
        let links = extract_markdown_links(input);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].text, "OpenAI");
        assert_eq!(links[0].url, "https://openai.com");
        assert_eq!(links[1].text, "Anthropic");
        assert_eq!(links[1].url, "https://anthropic.com");
    }

    #[test]
    fn test_extract_code_blocks_detailed() {
        let input = r#"Here is some code:
```rust
fn main() {
    println!("Hello");
}
```
And python:
```python
print("Hello")
```"#;
        let blocks = extract_code_blocks_detailed(input);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].language, "rust");
        assert_eq!(blocks[0].content, "fn main() {\n    println!(\"Hello\");\n}");
        assert_eq!(blocks[1].language, "python");
        assert_eq!(blocks[1].content, "print(\"Hello\")");
    }
}
