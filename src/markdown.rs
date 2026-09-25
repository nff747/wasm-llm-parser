pub fn parse_header(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_header() {
        assert_eq!(parse_header(""), "");
    }
}
