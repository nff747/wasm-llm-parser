use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_code_blocks(input: &str) -> Box<[JsValue]> {
    let mut blocks = Vec::new();
    let mut in_code_block = false;
    let mut current_block = String::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_code_block {
                blocks.push(JsValue::from_str(current_block.trim_end()));
                current_block.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }

    blocks.into_boxed_slice()
}

#[wasm_bindgen]
pub fn strip_markdown(input: &str) -> String {
    let mut stripped = String::with_capacity(input.len());
    let mut in_code_block = false;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        if !in_code_block {
            if !stripped.is_empty() {
                stripped.push('\n');
            }
            // Basic markdown stripping (headers, bold, italic)
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    '*' | '_' | '`' | '#' | '>' => continue,
                    '[' => continue,
                    ']' => continue,
                    _ => stripped.push(c)
                }
            }
        }
    }
    
    stripped
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_extract() {
        let md = "Some text\n```rust\nfn main() {}\n```\nMore text\n```\npython\n```";
        let blocks = extract_code_blocks(md);
        assert_eq!(blocks.len(), 2);
    }

    #[wasm_bindgen_test]
    fn test_strip() {
        let md = "# Hello\n**Bold** text\n```\ncode\n```\nMore text";
        let stripped = strip_markdown(md);
        assert_eq!(stripped, " Hello\nBold text\nMore text");
    }
}
