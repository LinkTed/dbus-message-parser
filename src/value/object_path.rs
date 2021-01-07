use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Split;
use thiserror::Error;

enum Input {
    /// [A-Z][a-z][0-9]_
    AlphanumericAndUnderscore,
    /// /
    Slash,
}

impl TryFrom<u8> for Input {
    type Error = ObjectPathError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c.is_ascii_alphanumeric() || c == b'_' {
            Ok(Input::AlphanumericAndUnderscore)
        } else if c == b'/' {
            Ok(Input::Slash)
        } else {
            Err(ObjectPathError::InvalidChar(c))
        }
    }
}

enum State {
    /// Start state.
    Start,
    /// The root slash.
    Root,
    /// The begining of the first or subsequent element.
    ElementBegin,
    /// The second or subsequent element.
    Element,
}

impl State {
    fn consume(self, i: Input) -> Result<State, ObjectPathError> {
        match self {
            State::Start => match i {
                Input::AlphanumericAndUnderscore => {
                    Err(ObjectPathError::BeginAlphanumericAndUnderscoreAndHyphen)
                }
                Input::Slash => Ok(State::Root),
            },
            State::Root => match i {
                Input::AlphanumericAndUnderscore => Ok(State::Element),
                Input::Slash => return Err(ObjectPathError::ElementEmtpy),
            },
            State::ElementBegin => match i {
                Input::AlphanumericAndUnderscore => Ok(State::Element),
                Input::Slash => return Err(ObjectPathError::ElementEmtpy),
            },
            State::Element => match i {
                Input::AlphanumericAndUnderscore => Ok(State::Element),
                Input::Slash => Ok(State::ElementBegin),
            },
        }
    }
}

/// Check if the given bytes is a valid [object path].
///
/// [object path]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path
fn check(error: &[u8]) -> Result<(), ObjectPathError> {
    let mut state = State::Start;
    for c in error {
        let i = Input::try_from(*c)?;
        state = state.consume(i)?;
    }

    match state {
        State::Start => Err(ObjectPathError::Empty),
        State::Root => Ok(()),
        State::ElementBegin => Err(ObjectPathError::EndSlash),
        State::Element => Ok(()),
    }
}

/// This represents a [object path].
///
/// [object path]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ObjectPath(String);

/// An enum representing all errors, which can occur during the handling of a [`ObjectPath`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum ObjectPathError {
    #[error("ObjectPath must not begin with an alphanumeric or with a '_' or with a '-'")]
    BeginAlphanumericAndUnderscoreAndHyphen,
    #[error("ObjectPath must not end with '.'")]
    EndSlash,
    #[error("ObjectPath must not be empty")]
    Empty,
    #[error("ObjectPath element must not be empty")]
    ElementEmtpy,
    #[error("ObjectPath must only contain '[A-Z][a-z][0-9]_/': {0}")]
    InvalidChar(u8),
}

impl From<ObjectPath> for String {
    fn from(object_path: ObjectPath) -> Self {
        object_path.0
    }
}

impl TryFrom<String> for ObjectPath {
    type Error = ObjectPathError;

    fn try_from(object_path: String) -> Result<Self, Self::Error> {
        check(object_path.as_bytes())?;
        Ok(ObjectPath(object_path))
    }
}

impl TryFrom<&str> for ObjectPath {
    type Error = ObjectPathError;

    fn try_from(object_path: &str) -> Result<Self, Self::Error> {
        check(object_path.as_bytes())?;
        Ok(ObjectPath(object_path.to_owned()))
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
        for c in element.as_bytes() {
            if !c.is_ascii_alphanumeric() && *c != b'_' {
                return false;
            }
        }
        if self.0 != "/" {
            self.0 += "/";
        }
        self.0 += element;
        true
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
