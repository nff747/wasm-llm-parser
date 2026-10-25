//! Streaming token delimiter extractor and bracket balancing validator for LLM completions.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonBracketState {
    Balanced,
    UnclosedObject,
    UnclosedArray,
    Mismatched,
}

#[wasm_bindgen]
pub struct StreamJsonValidator {
    depth_brace: i32,
    depth_bracket: i32,
    in_string: bool,
    is_escaped: bool,
}

#[wasm_bindgen]
impl StreamJsonValidator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            depth_brace: 0,
            depth_bracket: 0,
            in_string: false,
            is_escaped: false,
        }
    }

    pub fn feed_chunk(&mut self, chunk: &str) {
        for ch in chunk.chars() {
            if self.is_escaped {
                self.is_escaped = false;
                continue;
            }
            if ch == '\\' && self.in_string {
                self.is_escaped = true;
                continue;
            }
            if ch == '"' {
                self.in_string = !self.in_string;
                continue;
            }
            if self.in_string {
                continue;
            }

            match ch {
                '{' => self.depth_brace += 1,
                '}' => {
                    if self.depth_brace > 0 {
                        self.depth_brace -= 1;
                    }
                }
                '[' => self.depth_bracket += 1,
                ']' => {
                    if self.depth_bracket > 0 {
                        self.depth_bracket -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn state(&self) -> JsonBracketState {
        if self.depth_brace > 0 {
            JsonBracketState::UnclosedObject
        } else if self.depth_bracket > 0 {
            JsonBracketState::UnclosedArray
        } else {
            JsonBracketState::Balanced
        }
    }

    pub fn is_balanced(&self) -> bool {
        self.depth_brace == 0 && self.depth_bracket == 0 && !self.in_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_json_validator_complete() {
        let mut validator = StreamJsonValidator::new();
        validator.feed_chunk("{\"key\": \"val");
        assert!(!validator.is_balanced());
        validator.feed_chunk("ue\", \"count\": [1, 2, 3]}");
        assert!(validator.is_balanced());
        assert_eq!(validator.state(), JsonBracketState::Balanced);
    }

    #[test]
    fn test_stream_json_validator_unclosed() {
        let mut validator = StreamJsonValidator::new();
        validator.feed_chunk("{\"items\": [");
        assert_eq!(validator.state(), JsonBracketState::UnclosedObject);
    }
}
