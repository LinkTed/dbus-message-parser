use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Split;
use thiserror::Error;

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
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ObjectPath(String);

/// An enum representing all errors, which can occur during the handling of a [`ObjectPath`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum ObjectPathError {
    /// This error occurs, when the given string was not a valid object path.
    #[error("Length has the wrong length: {0}")]
    Regex(String),
}

impl From<ObjectPath> for String {
    fn from(object_path: ObjectPath) -> Self {
        object_path.0
    }
}

impl TryFrom<String> for ObjectPath {
    type Error = ObjectPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if OBJECT_PATH_REGEX.is_match(&value) {
            Ok(ObjectPath(value))
        } else {
            Err(ObjectPathError::Regex(value))
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

impl Display for ObjectPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ObjectPath {
    fn as_ref(&self) -> &str {
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
        self.as_ref() == other
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
    /// # use dbus_message_parser::value::ObjectPath;
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
    /// # use dbus_message_parser::value::ObjectPath;
    /// #
    /// let base = ObjectPath::try_from("/object").unwrap();
    ///
    /// let path_1 = ObjectPath::try_from("/object/path").unwrap();
    /// let path_2 = ObjectPath::try_from("/object_/path").unwrap();
    ///
    /// assert!(path_1.starts_with(&base));
    /// assert!(!path_2.starts_with(&base));
    /// assert!(!base.starts_with(&base));
    /// ```
    pub fn starts_with(&self, base: &ObjectPath) -> bool {
        if let Some(mut iter) = self.strip_prefix_elements(base) {
            iter.next().is_some()
        } else {
            false
        }
    }

    /// Returns a [`Split`] object with the prefix removed.
    ///
    /// [`Split`]: std::str::Split
    ///
    /// # Example
    /// ```
    /// # use std::convert::TryFrom;
    /// # use dbus_message_parser::value::ObjectPath;
    /// #
    /// let base = ObjectPath::try_from("/object").unwrap();
    ///
    /// let path_1 = ObjectPath::try_from("/object/path").unwrap();
    /// let path_2 = ObjectPath::try_from("/object_/path").unwrap();
    /// let path_3 = ObjectPath::try_from("/object/path/element").unwrap();
    ///
    /// let path_1_base_vec: Vec<&str> = path_1.strip_prefix_elements(&base).unwrap().collect();
    /// let path_3_base_vec: Vec<&str> = path_3.strip_prefix_elements(&base).unwrap().collect();
    ///
    /// assert_eq!(path_1_base_vec, vec!["path"]);
    /// assert!(path_2.strip_prefix_elements(&base).is_none());
    /// assert_eq!(path_3_base_vec, vec!["path", "element"]);
    /// assert!(base.strip_prefix_elements(&base).is_none());
    /// ```
    pub fn strip_prefix_elements<'a, 'b>(
        &'a self,
        base: &'b ObjectPath,
    ) -> Option<Split<'a, char>> {
        let mut self_iter = self.0.split('/');
        if self != "/" && base == "/" {
            self_iter.next()?;
            return Some(self_iter);
        }
        let mut base_iter = base.0.split('/');
        loop {
            let self_iter_prev = self_iter.clone();
            match (self_iter.next(), base_iter.next()) {
                (Some(ref x), Some(ref y)) => {
                    if x != y {
                        return None;
                    }
                }
                (Some(_), None) => return Some(self_iter_prev),
                (None, None) => return None,
                (None, Some(_)) => return None,
            }
        }
    }
}
