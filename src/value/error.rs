use crate::value::MAXIMUM_NAME_LENGTH;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error as ThisError;

enum Input {
    /// [A-Z][a-z]_
    AlphabeticAndUnderscore,
    /// [0-9]
    Digit,
    /// .
    Dot,
}

impl TryFrom<u8> for Input {
    type Error = ErrorError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphabetic() || c == b'_' {
            Ok(Input::AlphabeticAndUnderscore)
        } else if c.is_ascii_digit() {
            Ok(Input::Digit)
        } else if c == b'.' {
            Ok(Input::Dot)
        } else {
            Err(ErrorError::InvalidChar(c))
        }
    }
}

enum State {
    /// The first character of the first element.
    FirstElementBegin,
    /// The second or subsequent character of the first element.
    FirstElement,
    /// The first character of the second or subsequent element.
    ElementBegin,
    /// The second or subsequent character of the second or subsequent element.
    Element,
}

impl State {
    #[inline]
    fn consume(self, i: Input) -> Result<State, ErrorError> {
        match self {
            State::FirstElementBegin => match i {
                Input::AlphabeticAndUnderscore => Ok(State::FirstElement),
                Input::Digit => Err(ErrorError::ElementBeginDigit),
                Input::Dot => Err(ErrorError::ElementBeginDot),
            },
            State::FirstElement => match i {
                Input::AlphabeticAndUnderscore => Ok(State::FirstElement),
                Input::Digit => Ok(State::FirstElement),
                Input::Dot => Ok(State::ElementBegin),
            },
            State::ElementBegin => match i {
                Input::AlphabeticAndUnderscore => Ok(State::Element),
                Input::Digit => Err(ErrorError::ElementBeginDigit),
                Input::Dot => Err(ErrorError::ElementBeginDot),
            },
            State::Element => match i {
                Input::AlphabeticAndUnderscore => Ok(State::Element),
                Input::Digit => Ok(State::Element),
                Input::Dot => Ok(State::ElementBegin),
            },
        }
    }
}

/// Check if the given bytes is a valid [error name].
///
/// [error name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-error
fn check(error: &[u8]) -> Result<(), ErrorError> {
    let error_len = error.len();
    if MAXIMUM_NAME_LENGTH < error_len {
        return Err(ErrorError::ExceedMaximum(error_len));
    }

    let mut state = State::FirstElementBegin;
    for c in error {
        let i = Input::try_from(*c)?;
        state = state.consume(i)?;
    }

    match state {
        State::FirstElementBegin => Err(ErrorError::Empty),
        State::FirstElement => Err(ErrorError::Elements),
        State::ElementBegin => Err(ErrorError::EndDot),
        State::Element => Ok(()),
    }
}

/// This represents an [error name].
///
/// [error name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-error
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Error(String);

/// An enum representing all errors, which can occur during the handling of a [`Error`].
#[derive(Debug, PartialEq, Eq, ThisError)]
pub enum ErrorError {
    #[error("Error element must not begin with a digit")]
    ElementBeginDigit,
    #[error("Error element must not beign with a '.'")]
    ElementBeginDot,
    #[error("Error must not end with '.'")]
    EndDot,
    #[error("Error most not be empty")]
    Empty,
    #[error("Error have to be composed of 2 or more elements")]
    Elements,
    #[error("Error must not exceed the maximum length: {MAXIMUM_NAME_LENGTH} < {0}")]
    ExceedMaximum(usize),
    #[error("Error must only contain '[A-Z][a-z][0-9]_.': {0}")]
    InvalidChar(u8),
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        error.0
    }
}

impl TryFrom<String> for Error {
    type Error = ErrorError;

    fn try_from(error: String) -> Result<Self, Self::Error> {
        check(error.as_bytes())?;
        Ok(Error(error))
    }
}

impl TryFrom<&str> for Error {
    type Error = ErrorError;

    fn try_from(error: &str) -> Result<Self, Self::Error> {
        check(error.as_bytes())?;
        Ok(Error(error.to_owned()))
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
