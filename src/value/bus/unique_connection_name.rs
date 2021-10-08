use crate::value::MAXIMUM_NAME_LENGTH;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

enum Input {
    /// [A-Z][a-z][0-9]_-
    AlphanumericAndUnderscoreAndHyphen,
    /// .
    Dot,
    /// :
    Colon,
}

impl TryFrom<u8> for Input {
    type Error = UniqueConnectionNameError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphanumeric() || c == b'_' || c == b'-' {
            Ok(Input::AlphanumericAndUnderscoreAndHyphen)
        } else if c == b'.' {
            Ok(Input::Dot)
        } else if c == b':' {
            Ok(Input::Colon)
        } else {
            Err(UniqueConnectionNameError::InvalidChar(c))
        }
    }
}

enum State {
    /// The beginning of the first element.
    Start,
    ///
    BeginFirstElement,
    /// The second or subsequent character of the first element.
    FirstElement,
    /// The beginning of the second or subsequent element.
    Dot,
    /// The second or subsequent character of the second or subsequent element.
    SubsequentElement,
}

impl State {
    fn consume(self, i: Input) -> Result<State, UniqueConnectionNameError> {
        match self {
            State::Start => match i {
                Input::AlphanumericAndUnderscoreAndHyphen => {
                    Err(UniqueConnectionNameError::BeginColon)
                }
                Input::Dot => Err(UniqueConnectionNameError::BeginDot),
                Input::Colon => Ok(State::BeginFirstElement),
            },
            State::BeginFirstElement => match i {
                Input::AlphanumericAndUnderscoreAndHyphen => Ok(State::FirstElement),
                Input::Dot => Err(UniqueConnectionNameError::ElementBeginDot),
                Input::Colon => Err(UniqueConnectionNameError::Colon),
            },
            State::FirstElement => match i {
                Input::AlphanumericAndUnderscoreAndHyphen => Ok(State::FirstElement),
                Input::Dot => Ok(State::Dot),
                Input::Colon => Err(UniqueConnectionNameError::Colon),
            },
            State::Dot => match i {
                Input::AlphanumericAndUnderscoreAndHyphen => Ok(State::SubsequentElement),
                Input::Dot => Err(UniqueConnectionNameError::ElementBeginDot),
                Input::Colon => Err(UniqueConnectionNameError::Colon),
            },
            State::SubsequentElement => match i {
                Input::AlphanumericAndUnderscoreAndHyphen => Ok(State::SubsequentElement),
                Input::Dot => Ok(State::Dot),
                Input::Colon => Err(UniqueConnectionNameError::Colon),
            },
        }
    }
}

/// Check if the given bytes is a valid [unique connection name].
///
/// [unique connection name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
fn check(bus: &[u8]) -> Result<(), UniqueConnectionNameError> {
    let error_len = bus.len();
    if MAXIMUM_NAME_LENGTH < error_len {
        return Err(UniqueConnectionNameError::ExceedMaximum(error_len));
    }

    let mut state = State::Start;
    for c in bus {
        let i = Input::try_from(*c)?;
        state = state.consume(i)?;
    }

    match state {
        State::Start => Err(UniqueConnectionNameError::Empty),
        State::BeginFirstElement => Err(UniqueConnectionNameError::Empty),
        State::FirstElement => Err(UniqueConnectionNameError::Elements),
        State::Dot => Err(UniqueConnectionNameError::EndDot),
        State::SubsequentElement => Ok(()),
    }
}

/// This represents a [unique connection name].
///
/// [unique connection name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct UniqueConnectionName(String);

/// An enum representing all errors, which can occur during the handling of a
/// [`UniqueConnectionName`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum UniqueConnectionNameError {
    #[error("Unique connection name must begin with a ':'")]
    BeginColon,
    #[error("Unique connection name must not begin with a '.'")]
    BeginDot,
    #[error("Bus must not end with '.'")]
    EndDot,
    #[error("Bus element must not begin with a '.'")]
    ElementBeginDot,
    #[error("Colon have to be at the beginning")]
    Colon,
    #[error("Bus is empty")]
    Empty,
    #[error("Bus have to be composed of 2 or more elements")]
    Elements,
    #[error("Bus must not exceed the maximum length: {MAXIMUM_NAME_LENGTH} < {0}")]
    ExceedMaximum(usize),
    #[error("Bus must only contain '[A-Z][a-z][0-9]_-.:': {0}")]
    InvalidChar(u8),
}

impl From<UniqueConnectionName> for String {
    fn from(unique_connection_name: UniqueConnectionName) -> Self {
        unique_connection_name.0
    }
}

impl TryFrom<String> for UniqueConnectionName {
    type Error = UniqueConnectionNameError;

    fn try_from(unique_connection_name: String) -> Result<Self, Self::Error> {
        check(unique_connection_name.as_bytes())?;
        Ok(UniqueConnectionName(unique_connection_name))
    }
}

impl TryFrom<&str> for UniqueConnectionName {
    type Error = UniqueConnectionNameError;

    fn try_from(unique_connection_name: &str) -> Result<Self, Self::Error> {
        check(unique_connection_name.as_bytes())?;
        Ok(UniqueConnectionName(unique_connection_name.to_owned()))
    }
}

impl TryFrom<&[u8]> for UniqueConnectionName {
    type Error = UniqueConnectionNameError;

    fn try_from(unique_connection_name: &[u8]) -> Result<Self, Self::Error> {
        check(unique_connection_name)?;
        let unique_connection_name = unique_connection_name.to_vec();
        //  The vector only contains valid UTF-8 (ASCII) characters because it was already
        //  checked by the `check` function above
        unsafe {
            Ok(UniqueConnectionName(String::from_utf8_unchecked(
                unique_connection_name,
            )))
        }
    }
}

impl Display for UniqueConnectionName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for UniqueConnectionName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for UniqueConnectionName {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
