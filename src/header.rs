use crate::{DecodeError, Value, BUS_NAMES, INTERFACE_REGEX, MEMBER_REGEX, OBJECT_PATH_REGEX};
use std::convert::TryFrom;

/// An enum representing a [header field].
///
/// [header field]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
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

impl TryFrom<Value> for Header {
    type Error = DecodeError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
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
                        }
                        2 => {
                            // The header field is an Interface.
                            if let Value::String(s) = v {
                                if INTERFACE_REGEX.is_match(&s) {
                                    Ok(Header::Interface(s))
                                } else {
                                    Err(DecodeError::InterfaceRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        3 => {
                            // The header field is an Member.
                            if let Value::String(s) = v {
                                if MEMBER_REGEX.is_match(&s) {
                                    Ok(Header::Member(s))
                                } else {
                                    Err(DecodeError::MemberRegex)
                                }
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        4 => {
                            // The header field is an ErrorName.
                            if let Value::String(s) = v {
                                Ok(Header::ErrorName(s))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        5 => {
                            // The header field is a ReplySerial.
                            if let Value::Uint32(u) = v {
                                Ok(Header::ReplySerial(u))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
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
                        }
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
                        }
                        8 => {
                            // The header field is a Signature.
                            if let Value::Signature(s) = v {
                                Ok(Header::Signature(s))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
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
}
