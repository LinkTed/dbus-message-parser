use crate::MAXIMUM_NAME_LENGTH;
use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

lazy_static! {
    /// The regular expression for a valid [interface name].
    ///
    /// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
    pub static ref INTERFACE_REGEX: Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]+)+$").unwrap();
}

/// This represents an [interface name].
///
/// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Interface(String);

/// An enum representing all errors, which can occur during the handling of a [`Interface`].
#[derive(Debug, PartialEq, Eq)]
pub enum InterfaceError {
    /// This error occurs, when the given string was not a valid interface name.
    RegexError(String),
    /// This error occurs, when the given string has the wrong length.
    LengthError(usize),
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
                Err(InterfaceError::RegexError(value))
            }
        } else {
            Err(InterfaceError::LengthError(value_len))
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

impl Deref for Interface {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for Interface {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
