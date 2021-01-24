use crate::value::MAXIMUM_NAME_LENGTH;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

enum Input {
    /// [A-Z][a-z]_-
    AlphabeticAndUnderscoreAndHyphen,
    /// [0-9]_
    Digit,
    /// .
    Dot,
    /// :
    Colon,
}

impl TryFrom<u8> for Input {
    type Error = BusError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphabetic() || c == b'_' || c == b'-' {
            Ok(Input::AlphabeticAndUnderscoreAndHyphen)
        } else if c.is_ascii_digit() {
            Ok(Input::Digit)
        } else if c == b'.' {
            Ok(Input::Dot)
        } else if c == b':' {
            Ok(Input::Colon)
        } else {
            Err(BusError::InvalidChar(c))
        }
    }
}

enum State {
    /// Start state.
    Start,
    /// The beginning of the first element. (unique name)
    UniqueBegin,
    /// The second or subsequent character of the first element. (unique name)
    Unique,
    /// The beginning of the second or subsequent element. (unique name)
    UniqueDot,
    /// The second or subsequent character of the second or subsequent element. (unique name)
    Unique2,
    /// The first element. (well known name)
    WellKnown,
    /// The beginning of the second or subsequent element. (well known name)
    WellKnownDot,
    /// The second or subsequent character of the second or subsequent element. (well known name)
    WellKnown2,
}

impl State {
    fn consume(self, i: Input) -> Result<State, BusError> {
        match self {
            State::Start => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::WellKnown),
                Input::Digit => Err(BusError::BeginDigit),
                Input::Dot => Err(BusError::BeginDot),
                Input::Colon => Ok(State::UniqueBegin),
            },
            State::UniqueBegin => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::Unique),
                Input::Digit => Ok(State::Unique),
                Input::Dot => Err(BusError::ElementBeginDot),
                Input::Colon => Err(BusError::Colon),
            },
            State::Unique => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::Unique),
                Input::Digit => Ok(State::Unique),
                Input::Dot => Ok(State::UniqueDot),
                Input::Colon => Err(BusError::Colon),
            },
            State::UniqueDot => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::Unique2),
                Input::Digit => Ok(State::Unique2),
                Input::Dot => Err(BusError::ElementBeginDot),
                Input::Colon => Err(BusError::Colon),
            },
            State::Unique2 => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::Unique2),
                Input::Digit => Ok(State::Unique2),
                Input::Dot => Ok(State::UniqueDot),
                Input::Colon => Err(BusError::Colon),
            },
            State::WellKnown => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::WellKnown),
                Input::Digit => Ok(State::WellKnown),
                Input::Dot => Ok(State::WellKnownDot),
                Input::Colon => Err(BusError::Colon),
            },
            State::WellKnownDot => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::WellKnown2),
                Input::Digit => Err(BusError::WellKnownElementBeginDigit),
                Input::Dot => Err(BusError::ElementBeginDot),
                Input::Colon => Err(BusError::Colon),
            },
            State::WellKnown2 => match i {
                Input::AlphabeticAndUnderscoreAndHyphen => Ok(State::WellKnown2),
                Input::Digit => Ok(State::WellKnown2),
                Input::Dot => Ok(State::WellKnownDot),
                Input::Colon => Err(BusError::Colon),
            },
        }
    }
}

/// Check if the given bytes is a valid [bus name].
///
/// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
fn check(bus: &[u8]) -> Result<(), BusError> {
    let error_len = bus.len();
    if MAXIMUM_NAME_LENGTH < error_len {
        return Err(BusError::ExceedMaximum(error_len));
    }

    let mut state = State::Start;
    for c in bus {
        let i = Input::try_from(*c)?;
        state = state.consume(i)?;
    }

    match state {
        State::Start => Err(BusError::Empty),
        State::UniqueBegin => Err(BusError::UniqueEmpty),
        State::Unique => Err(BusError::Elements),
        State::UniqueDot => Err(BusError::EndDot),
        State::Unique2 => Ok(()),
        State::WellKnown => Err(BusError::Elements),
        State::WellKnownDot => Err(BusError::EndDot),
        State::WellKnown2 => Ok(()),
    }
}

/// This represents a [bus name].
///
/// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Bus(String);

/// An enum representing all errors, which can occur during the handling of a [`Bus`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum BusError {
    #[error("Bus must not begin with a digit")]
    BeginDigit,
    #[error("Bus must not begin with a '.'")]
    BeginDot,
    #[error("Bus must not end with '.'")]
    EndDot,
    #[error("Bus must not be empty")]
    UniqueEmpty,
    #[error("Bus element must not begin with a digit (well known)")]
    WellKnownElementBeginDigit,
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

impl From<Bus> for String {
    fn from(member: Bus) -> Self {
        member.0
    }
}

impl TryFrom<String> for Bus {
    type Error = BusError;

    fn try_from(bus: String) -> Result<Self, Self::Error> {
        check(bus.as_bytes())?;
        Ok(Bus(bus))
    }
}

impl TryFrom<&str> for Bus {
    type Error = BusError;

    fn try_from(bus: &str) -> Result<Self, Self::Error> {
        check(bus.as_bytes())?;
        Ok(Bus(bus.to_owned()))
    }
}

impl TryFrom<&[u8]> for Bus {
    type Error = BusError;

    fn try_from(bus: &[u8]) -> Result<Self, Self::Error> {
        check(bus)?;
        let bus = bus.to_vec();
        //  The vector only contains valid UTF-8 (ASCII) characters because it was already
        //  checked by the `check` function above
        unsafe { Ok(Bus(String::from_utf8_unchecked(bus))) }
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Bus {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Bus {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}
