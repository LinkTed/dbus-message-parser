use crate::value::MAXIMUM_NAME_LENGTH;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

enum Input {
    /// [A-Z][a-z]_-
    AlphabeticAndUnderscoreAndHyphen,
    /// [0-9]
    Digit,
    /// .
    Dot,
}

impl TryFrom<u8> for Input {
    type Error = WellKnownBusNameError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphabetic() || c == b'_' || c == b'-' {
            Ok(Input::AlphabeticAndUnderscoreAndHyphen)
        } else if c.is_ascii_digit() {
            Ok(Input::Digit)
        } else if c == b'.' {
            Ok(Input::Dot)
        } else {
            Err(WellKnownBusNameError::InvalidChar(c))
        }
    }
}

enum State {
    /// Start state.
    Start,
    /// The first element.
    FirstElement,
    /// The beginning of the second or subsequent element.
    Dot,
    /// The second or subsequent character of the second or subsequent element.
    SubsequentElement,
}

impl State {
    fn consume(self, i: Input) -> Result<State, WellKnownBusNameError> {
        match self {
            State::Start => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::FirstElement),
                Input::Digit => Err(WellKnownBusNameError::BeginDigit),
                Input::Dot => Err(WellKnownBusNameError::BeginDot),
            },
            State::FirstElement => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::FirstElement),
                Input::Digit => Ok(State::FirstElement),
                Input::Dot => Ok(State::Dot),
            },
            State::Dot => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::SubsequentElement),
                Input::Digit => Err(WellKnownBusNameError::ElementBeginDigit),
                Input::Dot => Err(WellKnownBusNameError::ElementBeginDot),
            },
            State::SubsequentElement => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::SubsequentElement),
                Input::Digit => Ok(State::SubsequentElement),
                Input::Dot => Ok(State::Dot),
            },
        }
    }
}

/// Check if the given bytes is a valid [well-known bus name].
///
/// [well-known bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
fn check(well_known_bus_name: &[u8]) -> Result<(), WellKnownBusNameError> {
    let error_len = well_known_bus_name.len();
    if MAXIMUM_NAME_LENGTH < error_len {
        return Err(WellKnownBusNameError::ExceedMaximum(error_len));
    }

    let mut state = State::Start;
    for c in well_known_bus_name {
        let i = Input::try_from(*c)?;
        state = state.consume(i)?;
    }

    match state {
        State::Start => Err(WellKnownBusNameError::Empty),
        State::FirstElement => Err(WellKnownBusNameError::Elements),
        State::Dot => Err(WellKnownBusNameError::EndDot),
        State::SubsequentElement => Ok(()),
    }
}

/// This represents a [well-known bus name].
///
/// [well-known bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct WellKnownBusName(String);

/// An enum representing all errors, which can occur during the handling of a [`WellKnownBusName`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum WellKnownBusNameError {
    #[error("Well-known bus name must not begin with a digit")]
    BeginDigit,
    #[error("Well-known bus name must not begin with a '.'")]
    BeginDot,
    #[error("Well-known bus name must not end with '.'")]
    EndDot,
    #[error("Well-known bus name element must not begin with a digit")]
    ElementBeginDigit,
    #[error("Well-known bus name element must not begin with a '.'")]
    ElementBeginDot,
    #[error("Well-known bus name is empty")]
    Empty,
    #[error("Well-known bus name have to be composed of 2 or more elements")]
    Elements,
    #[error("Well-known bus name must not exceed the maximum length: {MAXIMUM_NAME_LENGTH} < {0}")]
    ExceedMaximum(usize),
    #[error("Bus must only contain '[A-Z][a-z][0-9]_-.': {0}")]
    InvalidChar(u8),
}

impl From<WellKnownBusName> for String {
    fn from(well_known_bus_name: WellKnownBusName) -> Self {
        well_known_bus_name.0
    }
}

impl TryFrom<String> for WellKnownBusName {
    type Error = WellKnownBusNameError;

    fn try_from(well_known_bus_name: String) -> Result<Self, Self::Error> {
        check(well_known_bus_name.as_bytes())?;
        Ok(WellKnownBusName(well_known_bus_name))
    }
}

impl TryFrom<&str> for WellKnownBusName {
    type Error = WellKnownBusNameError;

    fn try_from(well_known_bus_name: &str) -> Result<Self, Self::Error> {
        check(well_known_bus_name.as_bytes())?;
        Ok(WellKnownBusName(well_known_bus_name.to_owned()))
    }
}

impl TryFrom<&[u8]> for WellKnownBusName {
    type Error = WellKnownBusNameError;

    fn try_from(well_known_bus_name: &[u8]) -> Result<Self, Self::Error> {
        check(well_known_bus_name)?;
        let well_known_bus_name = well_known_bus_name.to_vec();
        //  The vector only contains valid UTF-8 (ASCII) characters because it was already
        //  checked by the `check` function above
        unsafe {
            Ok(WellKnownBusName(String::from_utf8_unchecked(
                well_known_bus_name,
            )))
        }
    }
}

impl Display for WellKnownBusName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for WellKnownBusName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for WellKnownBusName {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
