pub(super) fn escape(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    for c in string.chars() {
        if c == '\'' {
            result.push_str("\\'");
        } else if c == ',' {
            result.push_str("','");
        } else if c == '\\' {
            result.push_str("'\\'");
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::escape;

    #[test]
    fn escape_1() {
        assert_eq!(escape("''"), "\\'\\'")
    }

    #[test]
    fn escape_2() {
        assert_eq!(escape(","), "','")
    }

    #[test]
    fn escape_3() {
        assert_eq!(escape("\\"), "'\\'")
    }

    #[test]
    fn escape_4() {
        assert_eq!(escape("a"), "a")
    }
}
