use crate::value::{
    Bus, BusError, Error, ErrorError, Interface, InterfaceError, Member, MemberError, ObjectPath,
    Type, Value,
};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error as ThisError;

#[derive(Debug, PartialEq, ThisError)]
pub enum MessageHeaderFieldError {
    #[error("Value is not a Struct: {0:?}")]
    Struct(Value),
    #[error("Struct does not contain exactly two values: {0}")]
    Length(usize),
    #[error("Second value is not a Variant: {0:?}")]
    Variant(Value),
    #[error("First value is not a Byte: {0:?}")]
    Byte(Value),
    #[error("Variant does not contain a ObjectPath: {0:?}")]
    Path(Value),
    #[error("Variant does not contain a String: {0:?}")]
    Interface(Value),
    #[error("String could not be converted to an Interface: {0}")]
    InterfaceError(#[from] InterfaceError),
    #[error("Variant does not contain a String: {0:?}")]
    Member(Value),
    #[error("String could not be converted to an Member: {0}")]
    MemberError(#[from] MemberError),
    #[error("Variant does not contain a String: {0:?}")]
    ErrorName(Value),
    #[error("String could not be converted to an ErrorName: {0}")]
    ErrorError(#[from] ErrorError),
    #[error("Variant does not contain a Uint32: {0:?}")]
    ReplySerial(Value),
    #[error("")]
    BusError(#[from] BusError),
    #[error("Variant does not contain a String: {0:?}")]
    Destination(Value),
    #[error("Variant does not contain a String: {0:?}")]
    Sender(Value),
    #[error("Variant does not contain a Signature: {0:?}")]
    Signature(Value),
    #[cfg(target_family = "unix")]
    #[error("Variant does not contain a Uint32: {0:?}")]
    UnixFDs(Value),
    #[error("The byte does not has a valid number: {0}")]
    InvalidNumber(u8),
}

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
    Signature(Vec<Type>),
    #[cfg(target_family = "unix")]
    UnixFDs(u32),
}

impl Display for MessageHeaderField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MessageHeaderField::Path(object_path) => write!(f, "path='{}'", object_path),
            MessageHeaderField::Interface(interface) => write!(f, "interface='{}'", interface),
            MessageHeaderField::Member(member) => write!(f, "member='{}'", member),
            MessageHeaderField::ErrorName(error_name) => write!(f, "error_name='{}'", error_name),
            MessageHeaderField::ReplySerial(reply_serial) => {
                write!(f, "reply_serial='{}'", reply_serial)
            }
            MessageHeaderField::Destination(destination) => {
                write!(f, "destination='{}'", destination)
            }
            MessageHeaderField::Sender(sender) => write!(f, "sender='{}'", sender),
            MessageHeaderField::Signature(signature) => write!(f, "signature='{:?}'", signature),
            #[cfg(target_family = "unix")]
            MessageHeaderField::UnixFDs(unix_fds) => write!(f, "unix_fds='{}'", unix_fds),
        }
    }
}

impl TryFrom<Value> for MessageHeaderField {
    type Error = MessageHeaderFieldError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        // The outer `Value` has to be a struct.
        let mut values: Vec<Value> = match value {
            Value::Struct(struct_) => struct_.into(),
            v => return Err(MessageHeaderFieldError::Struct(v)),
        };
        // The length of the struct have to be 2
        let values_len = values.len();
        if values_len != 2 {
            return Err(MessageHeaderFieldError::Length(values_len));
        }
        // Check if the second is a Variant and unwrap the value.
        let v = match values.pop().unwrap() {
            Value::Variant(v) => *v,
            v => return Err(MessageHeaderFieldError::Variant(v)),
        };
        // Check if the first is a byte
        let b = match values.pop().unwrap() {
            Value::Byte(b) => b,
            v => return Err(MessageHeaderFieldError::Byte(v)),
        };

        match b {
            // The header field is a Path.
            1 => match v {
                Value::ObjectPath(o) => Ok(MessageHeaderField::Path(o)),
                v => Err(MessageHeaderFieldError::Path(v)),
            },
            // The header field is an Interface.
            2 => match v {
                Value::String(s) => {
                    let interface = Interface::try_from(s)?;
                    Ok(MessageHeaderField::Interface(interface))
                }
                v => Err(MessageHeaderFieldError::Interface(v)),
            },
            // The header field is an Member.
            3 => match v {
                Value::String(s) => {
                    let member = Member::try_from(s)?;
                    Ok(MessageHeaderField::Member(member))
                }
                v => Err(MessageHeaderFieldError::Member(v)),
            },
            // The header field is an ErrorName.
            4 => match v {
                Value::String(s) => {
                    let error = Error::try_from(s)?;
                    Ok(MessageHeaderField::ErrorName(error))
                }
                v => Err(MessageHeaderFieldError::ErrorName(v)),
            },
            // The header field is a ReplySerial.
            5 => match v {
                Value::Uint32(u) => Ok(MessageHeaderField::ReplySerial(u)),
                v => Err(MessageHeaderFieldError::ReplySerial(v)),
            },
            // The header field is a Destination.
            6 => match v {
                Value::String(s) => {
                    let destination = Bus::try_from(s)?;
                    Ok(MessageHeaderField::Destination(destination))
                }
                v => Err(MessageHeaderFieldError::Destination(v)),
            },
            // The header field is a Sender.
            7 => match v {
                Value::String(s) => {
                    let sender = Bus::try_from(s)?;
                    Ok(MessageHeaderField::Sender(sender))
                }
                v => Err(MessageHeaderFieldError::Sender(v)),
            },
            // The header field is a Signature.
            8 => match v {
                Value::Signature(s) => Ok(MessageHeaderField::Signature(s)),
                v => Err(MessageHeaderFieldError::Signature(v)),
            },
            #[cfg(target_family = "unix")]
            9 => {
                // The header field is a UnixFds.
                match v {
                    Value::Uint32(u) => Ok(MessageHeaderField::UnixFDs(u)),
                    v => Err(MessageHeaderFieldError::UnixFDs(v)),
                }
            }
            // Invalid number.
            b => Err(MessageHeaderFieldError::InvalidNumber(b)),
        }
    }
}
