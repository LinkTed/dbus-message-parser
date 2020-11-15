use crate::{Bus, DecodeError, Error, Interface, Member, ObjectPath, Value};
use std::convert::TryFrom;

/// An enum representing a [header field].
///
/// [header field]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum MessageHeaderField {
    Path(ObjectPath),
    Interface(Interface),
    Member(Member),
    ErrorName(Error),
    ReplySerial(u32),
    Destination(Bus),
    Sender(Bus),
    Signature(String),
    #[cfg(target_family = "unix")]
    UnixFDs(u32),
}

impl TryFrom<Value> for MessageHeaderField {
    type Error = DecodeError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        // The outer `Value` has to be a struct.
        if let Value::Struct(mut values) = v {
            // The length of the struct have to be 2
            if values.len() == 2 {
                // Check if the second is a Variant and unwrap the value.
                let v: Value = if let Value::Variant(v) = values.pop().unwrap() {
                    *v
                } else {
                    return Err(DecodeError::Header);
                };
                // Check if the first is a byte
                if let Value::Byte(b) = values.pop().unwrap() {
                    match b {
                        1 => {
                            // The header field is a Path.
                            if let Value::ObjectPath(o) = v {
                                Ok(MessageHeaderField::Path(o))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        2 => {
                            // The header field is an Interface.
                            if let Value::String(s) = v {
                                Ok(MessageHeaderField::Interface(Interface::try_from(s)?))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        3 => {
                            // The header field is an Member.
                            if let Value::String(s) = v {
                                Ok(MessageHeaderField::Member(Member::try_from(s)?))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        4 => {
                            // The header field is an ErrorName.
                            if let Value::String(s) = v {
                                Ok(MessageHeaderField::ErrorName(Error::try_from(s)?))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        5 => {
                            // The header field is a ReplySerial.
                            if let Value::Uint32(u) = v {
                                Ok(MessageHeaderField::ReplySerial(u))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        6 => {
                            // The header field is a Destination.
                            if let Value::String(s) = v {
                                Ok(MessageHeaderField::Destination(Bus::try_from(s)?))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        7 => {
                            // The header field is a Sender.
                            if let Value::String(s) = v {
                                Ok(MessageHeaderField::Sender(Bus::try_from(s)?))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        8 => {
                            // The header field is a Signature.
                            if let Value::Signature(s) = v {
                                Ok(MessageHeaderField::Signature(s))
                            } else {
                                Err(DecodeError::Header)
                            }
                        }
                        #[cfg(target_family = "unix")]
                        9 => {
                            // The header field is a UnixFds.
                            if let Value::Uint32(u) = v {
                                Ok(MessageHeaderField::UnixFDs(u))
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
