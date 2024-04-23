pub fn extract_while<F>(accept: F, s: &str, error_msg: Option<String>) -> Result<(&str, &str), String>
where
    F: Fn(char) -> bool,
{
    let extracted_end = s.chars().take_while(|&c| accept(c)).count();
    if extracted_end == 0 {
        if let Some(msg) = error_msg {
            return Err(msg);
        } else {
            return Ok((s, ""));
        }
    }

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    Ok((remainder, extracted))
}

pub fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    extract_while(|c| c.is_ascii_digit(), s, Some("expected digits".to_string()))
}

pub fn extract_whitespace(s: &str, error_msg: Option<String>) -> Result<(&str, &str), String> {
    let (remainder, extracted) = extract_while(char::is_whitespace, s, error_msg)?;
    if let Some(first_non_whitespace) = remainder.find(|c: char| !c.is_whitespace()) {
        Ok((&remainder[first_non_whitespace..], extracted))
    } else {
        Ok((remainder, extracted))
    }
}

pub fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alpha = s.chars().next().map_or(false, |c| c.is_ascii_alphabetic());

    if input_starts_with_alpha {
        extract_while(char::is_alphanumeric, s, Some("expected identifier".to_string()))
    } else {
        Err("expected identifier".to_string())
    }
}

pub fn remove_tag<'a>(tag: &'a str, s: &'a str) -> Result<&'a str, String> {
    if let Some(stripped) = s.strip_prefix(tag) {
        Ok(stripped)
    } else {
        Err(format!("Expected '{}'", tag))
    }
}

pub fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator: impl Fn(&str, Option<String>) -> Result<(&str, T), String>,
    s: &str,
    error_msg: Option<String>,
) -> Result<(&str, Vec<T>), String> {
    let mut items = Vec::new();
    let mut s = s;

    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);

        let (new_s, _) = separator(s, None)?;
        s = new_s;
    }

    if let Some(error) = &error_msg {
        if items.is_empty() {
            return Err(error.clone());
        }
    }

    Ok((s, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1", None), Ok(("1", "    ")));
    }

    #[test]
    fn do_not_extract_spaces_start_when_input_does_not_start_with_them() {
        assert_eq!(extract_whitespace("blah", None), Ok(("blah", "")));
    }

    #[test]
    fn extract_alphabetic_identifier() {
        assert_eq!(extract_ident("abcdEFG stop"), Ok((" stop", "abcdEFG")));
    }

    #[test]
    fn extract_alphanumeric_identifier() {
        assert_eq!(extract_ident("foobar1()"), Ok(("()", "foobar1")));
    }

    #[test]
    fn cannot_extract_identifier_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), Err("expected identifier".to_string()),);
    }

    #[test]
    fn remove_tag_word() {
        assert_eq!(remove_tag("let", "let a"), Ok(" a"));
    }

    #[test]
    fn sequence_with_error_msg() {
        fn parser(s: &str) -> Result<(&str, usize), String> {
            if let Some(c) = s.chars().next() {
                if c.is_digit(10) {
                    return Ok((&s[1..], c.to_digit(10).unwrap() as usize));
                }
            }
            Err("Expected digit".to_string())
        }

        fn separator(s: &str, _error_msg: Option<String>) -> Result<(&str, usize), String> {
            if let Some(',') = s.chars().next() {
                return Ok((&s[1..], 0));
            }
            Ok(("", 0))
        }

        // Test case where the sequence contains items
        assert_eq!(sequence(parser, separator, "1,2,3", None), Ok(("", vec![1, 2, 3])));

        // Test case where the sequence is empty
        assert_eq!(
            sequence(parser, separator, "", Some("Empty sequence".to_string())),
            Err("Empty sequence".to_string())
        );

        // Test case where the input does not match the parser
        assert_eq!(
            sequence(parser, separator, "a,b,c", Some("Invalid input".to_string())),
            Err("Invalid input".to_string())
        );
    }
}
