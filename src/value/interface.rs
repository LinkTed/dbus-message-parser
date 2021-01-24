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
    /// .
    Dot,
}

impl TryFrom<u8> for Input {
    type Error = InterfaceError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphabetic() || c == b'_' {
            Ok(Input::AlphabeticAndUnderscore)
        } else if c.is_ascii_digit() {
            Ok(Input::Digit)
        } else if c == b'.' {
            Ok(Input::Dot)
        } else {
            Err(InterfaceError::InvalidChar(c))
        }
    }
}

enum State {
    /// The beginning of the first element.
    FirstElementBegin,
    /// The second or subsequent character of the first element.
    FirstElement,
    /// The beginning of the second or subsequent element.
    ElementBegin,
    /// The second or subsequent character of the second or subsequent element.
    Element,
}

impl State {
    #[inline]
    fn consume(self, i: Input) -> Result<State, InterfaceError> {
        match self {
            State::FirstElementBegin => match i {
                Input::AlphabeticAndUnderscore => Ok(State::FirstElement),
                Input::Digit => Err(InterfaceError::ElementBeginDigit),
                Input::Dot => Err(InterfaceError::ElementBeginDot),
            },
            State::FirstElement => match i {
                Input::AlphabeticAndUnderscore => Ok(State::FirstElement),
                Input::Digit => Ok(State::FirstElement),
                Input::Dot => Ok(State::ElementBegin),
            },
            State::ElementBegin => match i {
                Input::AlphabeticAndUnderscore => Ok(State::Element),
                Input::Digit => Err(InterfaceError::ElementBeginDigit),
                Input::Dot => Err(InterfaceError::ElementBeginDot),
            },
            State::Element => match i {
                Input::AlphabeticAndUnderscore => Ok(State::Element),
                Input::Digit => Ok(State::Element),
                Input::Dot => Ok(State::ElementBegin),
            },
        }
    }
}

/// Check if the given bytes is a valid [interface name].
///
/// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
fn check(interface: &[u8]) -> Result<(), InterfaceError> {
    let interface_len = interface.len();
    if MAXIMUM_NAME_LENGTH < interface_len {
        return Err(InterfaceError::ExceedMaximum(interface_len));
    }

    let mut state = State::FirstElementBegin;
    for c in interface {
        let i = Input::try_from(*c)?;
        state = state.consume(i)?;
    }

    match state {
        State::FirstElementBegin => Err(InterfaceError::Empty),
        State::FirstElement => Err(InterfaceError::Elements),
        State::ElementBegin => Err(InterfaceError::EndDot),
        State::Element => Ok(()),
    }
}

/// This represents an [interface name].
///
/// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Interface(String);

/// An enum representing all errors, which can occur during the handling of a [`Interface`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum InterfaceError {
    #[error("Interface element must not begin with a digit")]
    ElementBeginDigit,
    #[error("Interface element must not beign with a '.'")]
    ElementBeginDot,
    #[error("Interface must not end with '.'")]
    EndDot,
    #[error("Interface must not be empty")]
    Empty,
    #[error("Interface have to be composed of 2 or more elements")]
    Elements,
    #[error("Interface must not exceed the maximum length: {MAXIMUM_NAME_LENGTH} < {0}")]
    ExceedMaximum(usize),
    #[error("Interface must only contain '[A-Z][a-z][0-9]_.': {0}")]
    InvalidChar(u8),
}

impl From<Interface> for String {
    fn from(interface: Interface) -> Self {
        interface.0
    }
}

impl TryFrom<String> for Interface {
    type Error = InterfaceError;

    fn try_from(interface: String) -> Result<Self, Self::Error> {
        check(interface.as_bytes())?;
        Ok(Interface(interface))
    }
}

impl TryFrom<&str> for Interface {
    type Error = InterfaceError;

    fn try_from(interface: &str) -> Result<Self, Self::Error> {
        check(interface.as_bytes())?;
        Ok(Interface(interface.to_owned()))
    }
}

impl TryFrom<&[u8]> for Interface {
    type Error = InterfaceError;

    fn try_from(interface: &[u8]) -> Result<Self, Self::Error> {
        check(interface)?;
        let interface = interface.to_vec();
        //  The vector only contains valid UTF-8 (ASCII) characters because it was already
        //  checked by the `check` function above
        unsafe { Ok(Interface(String::from_utf8_unchecked(interface))) }
    }
}

impl Display for Interface {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Interface {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Interface {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
