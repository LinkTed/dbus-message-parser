use crate::value::MAXIMUM_NAME_LENGTH;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

lazy_static! {
    /// The regular expression for a valid [interface name].
    ///
    /// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
    pub static ref INTERFACE_REGEX: Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]+)+$").unwrap();
}

/// This represents an [interface name].
///
/// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Interface(String);

/// An enum representing all errors, which can occur during the handling of a [`Interface`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum InterfaceError {
    /// This error occurs, when the given string was not a valid interface name.
    #[error("Interface contains illegal character: {0}")]
    Regex(String),
    /// This error occurs, when the given string has the wrong length.
    #[error("Interface has the wrong length: {0}")]
    Length(usize),
}

impl From<Interface> for String {
    fn from(interface: Interface) -> Self {
        interface.0
    }
}

impl TryFrom<String> for Interface {
    type Error = InterfaceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_len = value.len();
        if 0 < value_len && value_len <= MAXIMUM_NAME_LENGTH {
            if INTERFACE_REGEX.is_match(&value) {
                Ok(Interface(value))
            } else {
                Err(InterfaceError::Regex(value))
            }
        } else {
            Err(InterfaceError::Length(value_len))
        }
    }
}

impl TryFrom<&str> for Interface {
    type Error = InterfaceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        Interface::try_from(value)
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
