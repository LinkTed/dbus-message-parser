use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

lazy_static! {
    /// The regular expression for a valid [member].
    ///
    /// [member]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
    pub static ref MEMBER_REGEX: Regex = Regex::new("^[A-Za-z_][A-Za-z0-9_]{0, 254}").unwrap();
}

/// This represents a [member].
///
/// [member]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Member(String);

/// An enum representing all errors, which can occur during the handling of a [`Member`].
#[derive(Debug, PartialEq, Eq)]
pub enum MemberError {
    /// This error occurs, when the given string was not a valid object path.
    TryFromError(String),
}

impl From<Member> for String {
    fn from(member: Member) -> Self {
        member.0
    }
}

impl TryFrom<String> for Member {
    type Error = MemberError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if MEMBER_REGEX.is_match(&value) {
            Ok(Member(value))
        } else {
            Err(MemberError::TryFromError(value))
        }
    }
}

impl TryFrom<&str> for Member {
    type Error = MemberError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        Member::try_from(value)
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Deref for Member {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Member {
    fn default() -> Self {
        Member("/".to_string())
    }
}

impl PartialEq<str> for Member {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
