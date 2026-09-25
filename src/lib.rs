use wasm_bindgen::prelude::*;

/// Scans a partial JSON string and returns the missing closing characters
/// (e.g., `"]}` or `}`) to make it structurally complete enough to parse.
#[wasm_bindgen]
pub fn heal_partial_json(input: &str) -> String {
    let mut stack = Vec::with_capacity(32);
    let mut in_string = false;
    let mut escape = false;

    for &b in input.as_bytes() {
        if in_string {
            if escape {
                escape = false;
                continue;
            }
            if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
        } else {
            match b {
                b'"' => in_string = true,
                b'{' => stack.push(b'}'),
                b'[' => stack.push(b']'),
                b'}' => {
                    if stack.last() == Some(&b'}') {
                        stack.pop();
                    }
                }
                b']' => {
                    if stack.last() == Some(&b']') {
                        stack.pop();
                    }
                }
                _ => {}
            }
        }
    }

    let mut missing = String::new();
    if in_string {
        if escape {
            missing.push('\"');
        }
        missing.push('"');
    }
    while let Some(c) = stack.pop() {
        missing.push(c as char);
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_heal_partial_json() {
        assert_eq!(heal_partial_json(r#"{"a": [1, 2"#), "]}");
        assert_eq!(heal_partial_json(r#"{"b": "hello"#), "\"}");
        assert_eq!(heal_partial_json(r#"{"c": {\"nested"#), "\"}}");
        assert_eq!(heal_partial_json(r#"["something", {"a": 1"#), "}]");
        assert_eq!(heal_partial_json(r#"{"done": true}"#), "");
        assert_eq!(heal_partial_json(r#"{"escaped\": true"#), "\"}");
    }
}
