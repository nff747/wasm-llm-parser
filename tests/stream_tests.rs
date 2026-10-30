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

#[test]
fn detects_crossed_and_unexpected_brackets() {
    for input in ["{]", "{[}]", "]", "}{"] {
        let mut val = StreamJsonValidator::new();
        val.feed_chunk(input);
        assert_eq!(val.state(), JsonBracketState::Mismatched);
        assert!(!val.is_balanced());
    }
}

#[test]
fn tracks_escaped_quotes_across_chunks_and_unclosed_strings() {
    let mut val = StreamJsonValidator::new();
    val.feed_chunk("{\"a\":\"one\\");
    val.feed_chunk("\" [still string]\"}");
    assert!(val.is_balanced());
    let mut string = StreamJsonValidator::new();
    string.feed_chunk("\"unfinished");
    assert_eq!(string.state(), JsonBracketState::UnclosedString);
    assert!(!string.is_balanced());
}
