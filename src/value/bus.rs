use crate::MAXIMUM_NAME_LENGTH;
use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

lazy_static! {
    /// The regular expression for a valid [bus name].
    ///
    /// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
    pub static ref BUS_REGEX: Regex = Regex::new(r"^((:[A-Za-z0-9_-]+(\.[A-Za-z0-9_-]+)*)|([A-Za-z_-][A-Za-z0-9_-]*(\.[A-Za-z_-][A-Za-z0-9_-]*)*))$").unwrap();
}

/// This represents a [bus name].
///
/// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Bus(String);

/// An enum representing all errors, which can occur during the handling of a [`Bus`].
#[derive(Debug, PartialEq, Eq)]
pub enum BusError {
    /// This error occurs, when the given string was not a valid bus name.
    RegexError(String),
    /// This error occurs, when the given string has the wrong length.
    LengthError(usize),
}

impl From<Bus> for String {
    fn from(member: Bus) -> Self {
        member.0
    }
}

impl TryFrom<String> for Bus {
    type Error = BusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_len = value.len();
        if 0 < value_len && value_len <= MAXIMUM_NAME_LENGTH {
            if BUS_REGEX.is_match(&value) {
                Ok(Bus(value))
            } else {
                Err(BusError::RegexError(value))
            }
        } else {
            Err(BusError::LengthError(value_len))
        }
    }
}

impl TryFrom<&str> for Bus {
    type Error = BusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        Bus::try_from(value)
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Deref for Bus {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for Bus {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
