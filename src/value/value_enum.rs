use crate::message::MessageHeaderField;
use crate::value::{ObjectPath, Signature, SignatureError};
use std::convert::TryInto;
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum Type {
    Byte,
    Boolean,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Double,
    String,
    ObjectPath,
    Signature,
    Array(Signature),
    Struct(Signature),
    DictEntry(Signature, Signature),
    Variant,
    #[cfg(target_family = "unix")]
    UnixFD,
}

impl Type {
    /// Returns the [alignment] of the [`Type`].
    ///
    /// [alignment]: https://dbus.freedesktop.org/doc/dbus-specification.html#idm702
    pub fn get_alignment(&self) -> usize {
        match self {
            Type::Byte => 1,
            Type::Boolean => 4,
            Type::Int16 => 2,
            Type::Uint16 => 2,
            Type::Int32 => 4,
            Type::Uint32 => 4,
            Type::Int64 => 8,
            Type::Uint64 => 8,
            Type::Double => 8,
            Type::String => 4,
            Type::ObjectPath => 4,
            Type::Signature => 1,
            Type::Array(_) => 4,
            Type::Struct(_) => 8,
            Type::DictEntry(_, _) => 8,
            Type::Variant => 1,
            #[cfg(target_family = "unix")]
            Type::UnixFD => 4,
        }
    }
}

/// An enum representing a [DBus value].
///
/// [DBus value]: https://dbus.freedesktop.org/doc/dbus-specification.html#type-system
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    Byte(u8),
    Boolean(bool),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Double(f64),
    String(String),
    ObjectPath(ObjectPath),
    Signature(Signature),
    Array(Vec<Value>, Signature),
    Struct(Vec<Value>),
    DictEntry(Box<(Value, Value)>),
    Variant(Box<Value>),
    #[cfg(target_family = "unix")]
    UnixFD(RawFd),
}

impl Value {
    /// Write the signature of the `Value` object into the `s` argument.
    pub(crate) fn get_signature_as_string(&self, s: &mut String) {
        match self {
            Value::Byte(_) => s.push('y'),
            Value::Boolean(_) => s.push('b'),
            Value::Int16(_) => s.push('n'),
            Value::Uint16(_) => s.push('q'),
            Value::Int32(_) => s.push('i'),
            Value::Uint32(_) => s.push('u'),
            Value::Int64(_) => s.push('x'),
            Value::Uint64(_) => s.push('t'),
            Value::Double(_) => s.push('d'),
            Value::String(_) => s.push('s'),
            Value::ObjectPath(_) => s.push('o'),
            Value::Signature(_) => s.push('g'),
            Value::Array(_, sig) => {
                s.push('a');
                s.push_str(sig.as_ref());
            }
            Value::Struct(vec) => {
                s.push('(');
                for v in vec {
                    v.get_signature_as_string(s);
                }
                s.push(')');
            }
            Value::DictEntry(b) => {
                s.push('{');
                let (key, value) = &**b;
                key.get_signature_as_string(s);
                value.get_signature_as_string(s);
                s.push('}');
            }
            Value::Variant(_) => s.push('v'),
            #[cfg(target_family = "unix")]
            Value::UnixFD(_) => s.push('h'),
        }
    }

    /// Returns the [`Signature`] of the `Value`.
    ///
    /// [`Signature`]: crate::value::Signature
    pub fn get_signature(&self) -> Result<Signature, SignatureError> {
        let mut signature = String::new();
        self.get_signature_as_string(&mut signature);
        signature.try_into()
    }
}

impl From<MessageHeaderField> for Value {
    fn from(header: MessageHeaderField) -> Self {
        let (b, v) = match header {
            MessageHeaderField::Path(s) => (Value::Byte(1), Value::ObjectPath(s)),
            MessageHeaderField::Interface(s) => (Value::Byte(2), Value::String(s.into())),
            MessageHeaderField::Member(s) => (Value::Byte(3), Value::String(s.into())),
            MessageHeaderField::ErrorName(s) => (Value::Byte(4), Value::String(s.into())),
            MessageHeaderField::ReplySerial(u) => (Value::Byte(5), Value::Uint32(u)),
            MessageHeaderField::Destination(s) => (Value::Byte(6), Value::String(s.into())),
            MessageHeaderField::Sender(s) => (Value::Byte(7), Value::String(s.into())),
            MessageHeaderField::Signature(s) => (Value::Byte(8), Value::Signature(s)),
            #[cfg(target_family = "unix")]
            MessageHeaderField::UnixFDs(u) => (Value::Byte(9), Value::Uint32(u)),
        };

        Value::Struct(vec![b, Value::Variant(Box::new(v))])
    }
}
