pub fn parse_header(input: &str) -> String {
    input.trim_start_matches('#').trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_header() {
        assert_eq!(parse_header(""), "");
    }

    #[test]
    fn test_h1_h2() {
        assert_eq!(parse_header("## Hello"), "Hello");
    }

    #[test]
    #[test]
    #[test]
    fn test_bold_multiple() {
        assert_eq!(parse_bold("**bold** and **more**"), "bold and more");
    }

    #[test]
    #[test]
    #[test]
    fn test_strip_custom_tag_1() {
        assert_eq!(strip_custom_tag_1("<tag1>text</tag1>"), "text");
    }

    #[test]
    #[test]
    fn test_strip_custom_tag_2() {
        assert_eq!(strip_custom_tag_2("<tag2>text</tag2>"), "text");
    }

    #[test]
    fn test_italic_basic() {
        assert_eq!(parse_italic("*italic*"), "italic");
    }

    #[test]
    fn test_bold_basic() {
        assert_eq!(parse_bold("**bold**"), "bold");
    }

    #[test]
    fn test_h1() {
        assert_eq!(parse_header("# Hello"), "Hello");
    }
}

pub fn parse_bold(input: &str) -> String {
    input.replace("**", "")
}

pub fn parse_italic(input: &str) -> String {
    input.replace("*", "")
}

pub fn strip_custom_tag_1(input: &str) -> String {
    input.replace("<tag1>", "").replace("</tag1>", "")
}

pub fn strip_custom_tag_2(input: &str) -> String {
    String::new()
}
