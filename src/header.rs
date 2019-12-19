use std::cmp::Ordering;
use regex::Regex;
use crate::{OBJECT_PATH_REGEX, Value, DecodeError};

/// An enum representing a [header field].
///
/// [header field]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields
#[derive(Debug, Eq, Clone, PartialOrd, Ord)]
pub enum Header {
    Path(String),
    Interface(String),
    Member(String),
    ErrorName(String),
    ReplySerial(u32),
    Destination(String),
    Sender(String),
    Signature(String),
}

impl PartialEq for Header {
    fn eq(&self, other: &Header) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Header {
    /// Parse a header field from a Value of the type `(yv)`.
    pub fn from(v: Value) -> Result<Header, DecodeError> {
        // The outer `Value` has to be a struct.
        if let Value::Struct(mut values) = v {
            // The length of the struct have to be 2
            if values.len() == 2 {
                // Check if the second is a Variant and unwrap the value.
                let v = if let Value::Variant(mut v) = values.pop().unwrap() {
                    // The Variant should only contain one element.
                    if v.len() != 1 {
                        return Err(DecodeError::Header);
                    }

                    v.pop().unwrap()
                } else {
                    return Err(DecodeError::Header);
                };
                // Check if the first is a byte
                if let Value::Byte(b) = values.pop().unwrap() {
                    lazy_static! {
                        /// The regular expression for a valid [bus name].
                        ///
                        /// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
                        static ref BUS_NAMES: Regex = Regex::new(":?[A-Za-z0-9_-]+(.[A-Za-z0-9_-]+)+").unwrap();
                    }

                    match b {
                        1 => {
                            // The header field is a Path.
                            if let Value::ObjectPath(s) = v {
                                if OBJECT_PATH_REGEX.is_match(&s) {
                                    Ok(Header::Path(s))
                                } else {
                                    Err(DecodeError::ObjectPathRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        2 => {
                            // The header field is an Interface.
                            if let Value::String(s) = v {
                                lazy_static! {
                                    /// The regular expression for a valid
                                    /// [interface name].
                                    ///
                                    /// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
                                    static ref INTERFACE_REGEX: Regex = Regex::new("[A-Za-z0-9_]+(.[A-Za-z0-9_]+)+").unwrap();
                                }

                                if INTERFACE_REGEX.is_match(&s) {
                                    Ok(Header::Interface(s))
                                } else {
                                    Err(DecodeError::InterfaceRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        3 => {
                            // The header field is an Interface.
                            if let Value::String(s) = v {
                                lazy_static! {
                                    /// The regular expression for a valid
                                    /// [member name].
                                    ///
                                    /// [member name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
                                    static ref MEMBER_REGEX: Regex = Regex::new("[A-Za-z_][A-Za-z0-9_]*").unwrap();
                                }

                                if MEMBER_REGEX.is_match(&s) {
                                    Ok(Header::Member(s))
                                } else {
                                    Err(DecodeError::MemberRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        4 => {
                            // The header field is an ErrorName.
                            if let Value::String(s) = v {
                                Ok(Header::ErrorName(s))
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        5 => {
                            // The header field is a ReplySerial.
                            if let Value::Uint32(u) = v {
                                Ok(Header::ReplySerial(u))
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        6 => {
                            // The header field is a Destination.
                            if let Value::String(s) = v {
                                if BUS_NAMES.is_match(&s) {
                                    Ok(Header::Destination(s))
                                } else {
                                    Err(DecodeError::BusNamesRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        7 => {
                            // The header field is a Sender.
                            if let Value::String(s) = v {
                                if BUS_NAMES.is_match(&s) {
                                    Ok(Header::Sender(s))
                                } else {
                                    Err(DecodeError::BusNamesRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        8 => {
                            // The header field is a Signature.
                            if let Value::Signature(s) = v {
                                Ok(Header::Signature(s))
                            } else {
                                Err(DecodeError::Header)
                            }
                        },
                        _ => {
                            // Invalid number.
                            Err(DecodeError::Header)
                        }
                    }
                } else {
                    Err(DecodeError::Header)
                }
            } else {
                Err(DecodeError::Header)
            }
        } else {
            Err(DecodeError::Header)
        }
    }

    /// Convert a `Header` object to a `Value`.
    pub fn into_value(self) -> Value {
        let (b, v) = match self {
            Header::Path(s) => {
                (Value::Byte(1), Value::ObjectPath(s))
            },
            Header::Interface(s) => {
                (Value::Byte(2), Value::String(s))
            },
            Header::Member(s) => {
                (Value::Byte(3), Value::String(s))
            },
            Header::ErrorName(s) => {
                (Value::Byte(4), Value::String(s))
            },
            Header::ReplySerial(u) => {
                (Value::Byte(5), Value::Uint32(u))
            },
            Header::Destination(s) => {
                (Value::Byte(6), Value::String(s))
            },
            Header::Sender(s) => {
                (Value::Byte(7), Value::String(s))
            },
            Header::Signature(s) => {
                (Value::Byte(8), Value::Signature(s))
            },
        };

        Value::Struct(vec![b, Value::Variant(vec![v])])
    }
}


#[cfg(test)]
use std::collections::BTreeSet;


#[test]
fn test_header_1() {
    let b = Value::Byte(1);
    let v = Value::Variant(vec![Value::ObjectPath("/".to_string())]);
    let header = Header::from(Value::Struct(vec![b, v])).unwrap();

    let mut tree = BTreeSet::new();
    tree.insert(header);

    let b = Value::Byte(1);
    let v = Value::Variant(vec![Value::ObjectPath("/a/".to_string())]);
    let header = Header::from(Value::Struct(vec![b, v])).unwrap();

    let mut tree = BTreeSet::new();
    tree.insert(header);

    assert_eq!(tree.len(), 1);
}
