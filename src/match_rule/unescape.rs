use super::MatchRuleError;

enum State {
    OutsideSingleQuotes,
    OutsideSingleQuotesBackslash,
    InsideSingleQuotes,
}

pub(super) fn unescape(string: &str) -> Result<String, MatchRuleError> {
    let mut result = String::new();
    let mut state = State::OutsideSingleQuotes;
    for c in string.chars() {
        match state {
            State::OutsideSingleQuotes => match c {
                '\\' => state = State::OutsideSingleQuotesBackslash,
                '\'' => state = State::InsideSingleQuotes,
                c => result.push(c),
            },
            State::OutsideSingleQuotesBackslash => match c {
                '\\' => result.push('\\'),
                '\'' => {
                    result.push('\'');
                    state = State::OutsideSingleQuotes
                }
                c => {
                    result.push('\\');
                    result.push(c);
                    state = State::OutsideSingleQuotes
                }
            },
            State::InsideSingleQuotes => match c {
                '\'' => state = State::OutsideSingleQuotes,
                c => result.push(c),
            },
        }
    }

    match state {
        State::OutsideSingleQuotes => Ok(result),
        State::OutsideSingleQuotesBackslash => {
            result.push('\\');
            Ok(result)
        }
        State::InsideSingleQuotes => Err(MatchRuleError::ValueClosingQuote),
    }
}

#[cfg(test)]
mod tests {
    use crate::match_rule::MatchRuleError;

    use super::unescape;

    #[test]
    fn unescape_1() {
        assert_eq!(unescape("\\").unwrap(), "\\")
    }

    #[test]
    fn unescape_2() {
        assert_eq!(unescape("\\'").unwrap(), "'")
    }

    #[test]
    fn unescape_3() {
        assert_eq!(unescape("\\\\").unwrap(), "\\\\")
    }

    #[test]
    fn unescape_4() {
        assert_eq!(unescape("\\a").unwrap(), "\\a")
    }

    #[test]
    fn unescape_5() {
        assert_eq!(unescape("'a'").unwrap(), "a")
    }

    #[test]
    fn unescape_error() {
        assert_eq!(unescape("'"), Err(MatchRuleError::ValueClosingQuote))
    }
}
