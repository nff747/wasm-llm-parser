use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_code_blocks(input: &str) -> String {
    let mut result = String::new();
    let mut in_block = false;
    for line in input.lines() {
        if line.starts_with("```") {
            in_block = !in_block;
            continue;
        }
        if in_block {
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
    fn test_single_block() {
        let input = "```rust\nfn main() {}\n```";
        assert_eq!(extract_code_blocks(input), "fn main() {}");
    }
}
