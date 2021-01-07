use crate::value::MAXIMUM_NAME_LENGTH;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

enum Input {
    /// [A-Z][a-z]_
    AlphabeticAndUnderscore,
    /// [0-9]
    Digit,
}

impl TryFrom<u8> for Input {
    type Error = MemberError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphabetic() || c == b'_' {
            Ok(Input::AlphabeticAndUnderscore)
        } else if c.is_ascii_digit() {
            Ok(Input::Digit)
        } else {
            Err(MemberError::InvalidChar(c))
        }
    }
}

/// Check if the given bytes is a valid [member name].
///
/// [member name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
fn check(member: &[u8]) -> Result<(), MemberError> {
    let member_len = member.len();
    if MAXIMUM_NAME_LENGTH < member_len {
        return Err(MemberError::ExceedMaximum(member_len));
    }

    let mut member_iter = member.iter();
    match member_iter.next() {
        Some(c) => {
            if let Input::Digit = Input::try_from(*c)? {
                return Err(MemberError::BeginDigit);
            }
        }
        None => return Err(MemberError::Empty),
    }

    while let Some(c) = member_iter.next() {
        Input::try_from(*c)?;
    }
    Ok(())
}

/// This represents a [member name].
///
/// [member name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Member(String);

/// An enum representing all errors, which can occur during the handling of a [`Member`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum MemberError {
    #[error("Member must not be empty")]
    Empty,
    #[error("Member must not begin with a digit")]
    BeginDigit,
    #[error("Member must not exceed the maximum length: {MAXIMUM_NAME_LENGTH} < {0}")]
    ExceedMaximum(usize),
    #[error("Member contians an invalid char: {0}")]
    InvalidChar(u8),
}

impl From<Member> for String {
    fn from(member: Member) -> Self {
        member.0
    }
}

impl TryFrom<String> for Member {
    type Error = MemberError;

    fn try_from(member: String) -> Result<Self, Self::Error> {
        check(member.as_bytes())?;
        Ok(Member(member))
    }
}

impl TryFrom<&str> for Member {
    type Error = MemberError;

    fn try_from(member: &str) -> Result<Self, Self::Error> {
        check(member.as_bytes())?;
        Ok(Member(member.to_owned()))
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Member {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Member {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
