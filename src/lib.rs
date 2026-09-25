use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_code_blocks(input: &str) -> String {
    input.trim().to_string()
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
}
