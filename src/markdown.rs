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
    fn test_h1() {
        assert_eq!(parse_header("# Hello"), "Hello");
    }
}
