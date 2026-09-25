use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_code_blocks(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_empty() {
        assert_eq!(extract_code_blocks(""), "");
    }
}

