mod decode;
mod encode;
mod member;
mod object_path;

use crate::Header;
pub use member::{Member, MemberError, MEMBER_REGEX};
pub use object_path::{ObjectPath, ObjectPathError, OBJECT_PATH_ELEMENT_REGEX, OBJECT_PATH_REGEX};
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

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
    Signature(String),
    Array(Vec<Value>, String),
    Struct(Vec<Value>),
    DictEntry(Box<(Value, Value)>),
    Variant(Box<Value>),
    #[cfg(target_family = "unix")]
    UnixFD(RawFd),
}

impl Value {
    /// Write the signature of the `Value` object into the `s` argument.
    pub fn get_signature(&self, s: &mut String) {
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
                s.push_str(sig);
            }
            Value::Struct(vec) => {
                s.push('(');
                for v in vec {
                    v.get_signature(s);
                }
                s.push(')');
            }
            Value::DictEntry(b) => {
                s.push('{');
                let (key, value) = &**b;
                key.get_signature(s);
                value.get_signature(s);
                s.push('}');
            }
            Value::Variant(_) => s.push('v'),
            #[cfg(target_family = "unix")]
            Value::UnixFD(_) => s.push('h'),
        }
    }
}

impl From<Header> for Value {
    fn from(header: Header) -> Self {
        let (b, v) = match header {
            Header::Path(s) => (Value::Byte(1), Value::ObjectPath(s)),
            Header::Interface(s) => (Value::Byte(2), Value::String(s)),
            Header::Member(s) => (Value::Byte(3), Value::String(s.into())),
            Header::ErrorName(s) => (Value::Byte(4), Value::String(s)),
            Header::ReplySerial(u) => (Value::Byte(5), Value::Uint32(u)),
            Header::Destination(s) => (Value::Byte(6), Value::String(s)),
            Header::Sender(s) => (Value::Byte(7), Value::String(s)),
            Header::Signature(s) => (Value::Byte(8), Value::Signature(s)),
        };

        Value::Struct(vec![b, Value::Variant(Box::new(v))])
    }
}
