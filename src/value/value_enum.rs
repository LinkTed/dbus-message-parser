use crate::value::{Array, ObjectPath, Struct, Type, TypeError};
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
    Signature(Vec<Type>),
    Array(Array),
    Struct(Struct),
    DictEntry(Box<(Value, Value)>),
    Variant(Box<Value>),
    #[cfg(target_family = "unix")]
    UnixFD(RawFd),
}

impl Value {
    /// Write the signature of the `Value` object into the `s` argument.
    pub(crate) fn to_signature_string(
        &self,
        signature_string: &mut String,
        array_depth: u8,
        struct_depth: u8,
        dict_depth: u8,
    ) -> Result<(), TypeError> {
        Type::check_depth(array_depth, struct_depth, dict_depth)?;
        match self {
            Value::Byte(_) => signature_string.push('y'),
            Value::Boolean(_) => signature_string.push('b'),
            Value::Int16(_) => signature_string.push('n'),
            Value::Uint16(_) => signature_string.push('q'),
            Value::Int32(_) => signature_string.push('i'),
            Value::Uint32(_) => signature_string.push('u'),
            Value::Int64(_) => signature_string.push('x'),
            Value::Uint64(_) => signature_string.push('t'),
            Value::Double(_) => signature_string.push('d'),
            Value::String(_) => signature_string.push('s'),
            Value::ObjectPath(_) => signature_string.push('o'),
            Value::Signature(_) => signature_string.push('g'),
            Value::Array(array) => {
                signature_string.push('a');
                array.get_type().try_to_string(
                    signature_string,
                    array_depth + 1,
                    struct_depth,
                    dict_depth,
                )?;
            }
            Value::Struct(struct_) => {
                signature_string.push('(');
                for v in struct_.as_ref() {
                    v.to_signature_string(
                        signature_string,
                        array_depth,
                        struct_depth + 1,
                        dict_depth,
                    )?;
                }
                signature_string.push(')');
            }
            Value::DictEntry(b) => {
                signature_string.push('{');
                let (key, value) = &**b;
                key.to_signature_string(
                    signature_string,
                    array_depth,
                    struct_depth,
                    dict_depth + 1,
                )?;
                value.to_signature_string(
                    signature_string,
                    array_depth,
                    struct_depth,
                    dict_depth + 1,
                )?;
                signature_string.push('}');
            }
            Value::Variant(_) => signature_string.push('v'),
            #[cfg(target_family = "unix")]
            Value::UnixFD(_) => signature_string.push('h'),
        }
        Type::check_len(signature_string)?;
        Ok(())
    }

    fn from_value_to_type(
        &self,
        array_depth: u8,
        struct_depth: u8,
        dict_depth: u8,
    ) -> Result<Type, TypeError> {
        Type::check_depth(array_depth, struct_depth, dict_depth)?;
        match self {
            Value::Byte(_) => Ok(Type::Byte),
            Value::Boolean(_) => Ok(Type::Boolean),
            Value::Int16(_) => Ok(Type::Int16),
            Value::Uint16(_) => Ok(Type::Uint16),
            Value::Int32(_) => Ok(Type::Int32),
            Value::Uint32(_) => Ok(Type::Uint32),
            Value::Int64(_) => Ok(Type::Int64),
            Value::Uint64(_) => Ok(Type::Uint64),
            #[cfg(target_family = "unix")]
            Value::UnixFD(_) => Ok(Type::UnixFD),
            Value::Double(_) => Ok(Type::Double),
            Value::String(_) => Ok(Type::String),
            Value::ObjectPath(_) => Ok(Type::ObjectPath),
            Value::Signature(_) => Ok(Type::Signature),
            Value::Array(array) => {
                let signature = Box::new(array.get_type().clone());
                Ok(Type::Array(signature))
            }
            Value::Struct(struct_) => {
                let mut signatures = Vec::new();
                for value in struct_.as_ref() {
                    let signature =
                        value.from_value_to_type(array_depth, struct_depth + 1, dict_depth)?;
                    signatures.push(signature);
                }
                Ok(Type::Struct(signatures))
            }
            Value::DictEntry(b) => {
                let key_signature =
                    b.0.from_value_to_type(array_depth, struct_depth, dict_depth + 1)?;
                let value_signature =
                    b.1.from_value_to_type(array_depth, struct_depth, dict_depth + 1)?;
                let signature = Box::new((key_signature, value_signature));
                Ok(Type::DictEntry(signature))
            }
            Value::Variant(_) => Ok(Type::Variant),
        }
    }

    /// Returns the [`Type`] of the `Value`.
    ///
    /// [`Type`]: crate::value::Type
    pub fn get_type(&self) -> Result<Type, TypeError> {
        self.from_value_to_type(0, 0, 0)
    }
}
