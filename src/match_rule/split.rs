use super::MatchRuleError;
use std::iter::Iterator;

pub struct Split<'a>(&'a str);

impl Split<'_> {
    pub(super) fn new(match_rules: &str) -> Split<'_> {
        Split(match_rules)
    }
}

enum Input {
    AlphanumericAndUnderscore,
    Equal,
    Comma,
    Quote,
    Backslash,
    Others,
}

impl From<char> for Input {
    fn from(c: char) -> Self {
        if c.is_ascii_alphanumeric() || c == '_' {
            Input::AlphanumericAndUnderscore
        } else if c == '=' {
            Input::Equal
        } else if c == ',' {
            Input::Comma
        } else if c == '\'' {
            Input::Quote
        } else if c == '\\' {
            Input::Backslash
        } else {
            Input::Others
        }
    }
}

enum State {
    KeyBegin,
    Key,
    Value,
    ValueBackslash,
    ValueInsideQuotes,
}

fn next_position(string: &str) -> Result<usize, MatchRuleError> {
    let mut state = State::KeyBegin;
    for (position, c) in string.char_indices() {
        let i = Input::from(c);
        match state {
            State::KeyBegin => match i {
                Input::AlphanumericAndUnderscore => state = State::Key,
                Input::Equal => return Err(MatchRuleError::KeyEmpty),
                _ => return Err(MatchRuleError::KeyInvalidChar(c)),
            },
            State::Key => match i {
                Input::AlphanumericAndUnderscore => state = State::Key,
                Input::Equal => state = State::Value,
                _ => return Err(MatchRuleError::KeyInvalidChar(c)),
            },
            State::Value => match i {
                Input::Backslash => state = State::ValueBackslash,
                Input::Quote => state = State::ValueInsideQuotes,
                Input::Comma => return Ok(position),
                _ => (),
            },
            State::ValueBackslash => match i {
                Input::Backslash => state = State::ValueBackslash,
                Input::Comma => return Ok(position),
                _ => state = State::Value,
            },
            State::ValueInsideQuotes => {
                if let Input::Quote = i {
                    state = State::Value;
                }
            }
        }
    }

    match state {
        State::KeyBegin => Err(MatchRuleError::KeyEmpty),
        State::Key => Err(MatchRuleError::MissingEqual),
        State::ValueBackslash => Ok(string.len()),
        State::ValueInsideQuotes => Err(MatchRuleError::ValueClosingQuote),
        State::Value => Ok(string.len()),
    }
}

impl<'a> Iterator for Split<'a> {
    type Item = Result<&'a str, MatchRuleError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        match next_position(self.0) {
            Ok(position) => {
                let (result, string) = self.0.split_at(position);
                if let Some(string) = string.strip_prefix(',') {
                    self.0 = string;
                } else {
                    self.0 = string;
                }
                Some(Ok(result))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{next_position, MatchRuleError, Split};

    #[test]
    fn split_1() {
        let mut split = Split::new("a=a,b=b");
        assert_eq!(split.next(), Some(Ok("a=a")));
        assert_eq!(split.next(), Some(Ok("b=b")));
        assert_eq!(split.next(), None)
    }

    #[test]
    fn split_2() {
        let mut split = Split::new("a=''\\,b=b");
        assert_eq!(split.next(), Some(Ok("a=''\\")));
        assert_eq!(split.next(), Some(Ok("b=b")));
        assert_eq!(split.next(), None)
    }

    #[test]
    fn split_3() {
        let mut split = Split::new("a=',\\',b=b");
        assert_eq!(split.next(), Some(Ok("a=',\\'")));
        assert_eq!(split.next(), Some(Ok("b=b")));
        assert_eq!(split.next(), None)
    }

    #[test]
    fn split_4() {
        let mut split = Split::new("a=\\\\");
        assert_eq!(split.next(), Some(Ok("a=\\\\")));
        assert_eq!(split.next(), None);
    }

    #[test]
    fn split_5() {
        let mut split = Split::new("a=\\a");
        assert_eq!(split.next(), Some(Ok("a=\\a")));
        assert_eq!(split.next(), None);
    }

    #[test]
    fn split_error_1() {
        let mut split = Split::new("=");
        assert_eq!(split.next(), Some(Err(MatchRuleError::KeyEmpty)))
    }

    #[test]
    fn split_error_2() {
        let mut split = Split::new("a");
        assert_eq!(split.next(), Some(Err(MatchRuleError::MissingEqual)))
    }

    #[test]
    fn split_error_3() {
        let mut split = Split::new("a='");
        assert_eq!(split.next(), Some(Err(MatchRuleError::ValueClosingQuote)))
    }

    #[test]
    fn split_error_4() {
        let mut split = Split::new("\\='");
        assert_eq!(
            split.next(),
            Some(Err(MatchRuleError::KeyInvalidChar('\\')))
        )
    }

    #[test]
    fn split_error_5() {
        let mut split = Split::new("a\\='");
        assert_eq!(
            split.next(),
            Some(Err(MatchRuleError::KeyInvalidChar('\\')))
        )
    }

    #[test]
    fn next_position_error() {
        assert_eq!(next_position(""), Err(MatchRuleError::KeyEmpty))
    }
}
