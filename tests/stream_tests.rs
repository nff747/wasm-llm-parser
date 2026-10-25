use wasm_llm_parser::stream::{StreamJsonValidator, JsonBracketState};

#[test]
fn test_multiline_json_stream_chunks() {
    let mut val = StreamJsonValidator::new();
    val.feed_chunk("{\n  \"model\": \"aira-70b\",\n");
    assert_eq!(val.state(), JsonBracketState::UnclosedObject);
    assert!(!val.is_balanced());

    val.feed_chunk("  \"tokens\": [101, 102]\n}");
    assert_eq!(val.state(), JsonBracketState::Balanced);
    assert!(val.is_balanced());
}
