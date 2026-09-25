use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_code_blocks(input: &str) -> String {
    let mut result = String::new();
    let mut in_block = false;
    for line in input.lines() {
        if line.starts_with("```") {
            if in_block {
                result.push('\n');
            }
            in_block = !in_block;
            continue;
        }
        if in_block {
            if !result.is_empty() && !result.ends_with('\n') { result.push('\n'); }
            result.push_str(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_empty() {
        assert_eq!(extract_code_blocks("   "), "");
        assert_eq!(extract_code_blocks(""), "");
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_multiple_blocks() {
        let input = "```rust\nlet x = 1;\n```\nText\n```python\nx = 1\n```";
        assert_eq!(extract_code_blocks(input), "let x = 1;\n\nx = 1");
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_single_block() {
        let input = "Some text\n```rust\nlet x = 1;\nlet y = 2;\n```\nMore text";
        assert_eq!(extract_code_blocks(input), "let x = 1;\nlet y = 2;");
        let input = "```rust\nfn main() {}\n```";
        assert_eq!(extract_code_blocks(input), "fn main() {}");
    }
}

#[wasm_bindgen]
pub fn parse_think_tags(input: &str) -> String {
    let mut result = String::new();
    let mut in_think = false;
    for line in input.lines() {
        if line.contains("<think>") { in_think = true; continue; }
        if line.contains("</think>") { in_think = false; continue; }
        if in_think { 
            if !result.is_empty() { result.push('\n'); }
            result.push_str(line);
        }
    }
    result
}
