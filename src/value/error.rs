use crate::value::MAXIMUM_NAME_LENGTH;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error as ThisError;

lazy_static! {
    /// The regular expression for a valid [error name].
    ///
    /// [error name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-error
    pub static ref ERROR_REGEX: Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]+)+$").unwrap();
}

/// This represents an [error name].
///
/// [error name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-error
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Error(String);

/// An enum representing all errors, which can occur during the handling of a [`Error`].
#[derive(Debug, PartialEq, Eq, ThisError)]
pub enum ErrorError {
    /// This error occurs, when the given string was not a valid error name.
    #[error("Error contains illegal character: {0}")]
    Regex(String),
    /// This error occurs, when the given string has the wrong length.
    #[error("Error has the wrong length: {0}")]
    Length(usize),
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        error.0
    }
}

impl TryFrom<String> for Error {
    type Error = ErrorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_len = value.len();
        if 0 < value_len && value_len <= MAXIMUM_NAME_LENGTH {
            if ERROR_REGEX.is_match(&value) {
                Ok(Error(value))
            } else {
                Err(ErrorError::Regex(value))
            }
        } else {
            Err(ErrorError::Length(value_len))
        }
    }
}

impl TryFrom<&str> for Error {
    type Error = ErrorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        Error::try_from(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Error {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
