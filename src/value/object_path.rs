use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::ops::Deref;

lazy_static! {
    /// The regular expression for a valid [object path].
    ///
    /// [object path]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path
    pub static ref OBJECT_PATH_REGEX: Regex = Regex::new(r"^/([A-Za-z0-9_]+(/[A-Za-z0-9_]+)*)?$").unwrap();

    /// The regular expression for a element of an [object path].
    ///
    /// [object path]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path
    pub static ref OBJECT_PATH_ELEMENT_REGEX: Regex = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
}

/// This represents a [object path].
///
/// [object path]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ObjectPath(String);

/// An enum representing all errors, which can occur during the handling of a [`ObjectPath`].
#[derive(Debug, PartialEq, Eq)]
pub enum ObjectPathError {
    /// This error occurs, when the given string was not a valid object path.
    TryFromError(String),
}

impl TryFrom<String> for ObjectPath {
    type Error = ObjectPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if OBJECT_PATH_REGEX.is_match(&value) {
            Ok(ObjectPath(value))
        } else {
            Err(ObjectPathError::TryFromError(value))
        }
    }
}

impl TryFrom<&str> for ObjectPath {
    type Error = ObjectPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        ObjectPath::try_from(value)
    }
}

impl ToString for ObjectPath {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Deref for ObjectPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ObjectPath {
    fn default() -> Self {
        ObjectPath("/".to_string())
    }
}

impl PartialEq<str> for ObjectPath {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl ObjectPath {
    /// Append an element to the object path.
    ///
    /// Returns `true` if the new element could be appended, otherwise `false`.
    /// An element cannot be appended if the element contain a character, which is not allowed.
    ///
    /// # Example
    /// ```
    /// # use std::convert::TryFrom;
    /// # use dbus_message_parser::ObjectPath;
    /// #
    /// let mut object_path = ObjectPath::try_from("/object").unwrap();
    ///
    /// assert!(object_path.append("path"));
    /// assert!(!object_path.append("/path"));
    ///
    /// assert_eq!(&object_path, "/object/path");
    /// ```
    pub fn append(&mut self, element: &str) -> bool {
        if OBJECT_PATH_ELEMENT_REGEX.is_match(element) {
            if self.0 != "/" {
                self.0 += "/";
            }
            self.0 += element;
            true
        } else {
            false
        }
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// # Example
    /// ```
    /// # use std::convert::TryFrom;
    /// # use dbus_message_parser::ObjectPath;
    /// #
    /// let base = ObjectPath::try_from("/object").unwrap();
    ///
    /// let path_1 = ObjectPath::try_from("/object/path").unwrap();
    /// let path_2 = ObjectPath::try_from("/object_/path").unwrap();
    ///
    /// assert!(path_1.start_with(&base));
    /// assert!(!path_2.start_with(&base));
    /// assert!(!base.start_with(&base));
    /// ```
    pub fn start_with(&self, base: &ObjectPath) -> bool {
        if self.0.starts_with(&base.0) {
            if let Some(c) = self.0.chars().nth(base.0.len()) {
                c == '/'
            } else {
                false
            }
        } else {
            false
        }
    }
}
