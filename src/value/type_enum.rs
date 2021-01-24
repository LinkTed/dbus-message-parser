use crate::value::{MAXIMUM_ARRAY_DEPTH, MAXIMUM_DICT_DEPTH, MAXIMUM_STRUCT_DEPTH};
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

pub const MAXIMUM_SIGNATURE_LENGTH: usize = 255;

/// An enum representing all errors, which can occur during the handling of a [`Signature`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum TypeError {
    #[error("Signature contians an invalid char: {0}")]
    InvalidChar(u8),
    #[error("Array depth is too big: {MAXIMUM_ARRAY_DEPTH} < {0}")]
    ArrayDepth(u8),
    #[error("Struct depth is too big: {MAXIMUM_STRUCT_DEPTH} < {0}")]
    StructDepth(u8),
    #[error("Dict depth is too big: {MAXIMUM_DICT_DEPTH} < {0}")]
    DictDepth(u8),
    #[error("Signature is too big: {MAXIMUM_SIGNATURE_LENGTH} < {0}")]
    TooShort(usize, usize),
    #[error("The closing curly bracket is missing for the dict at {0} got {1}")]
    ClosingCurlyBracket(usize, u8),
    #[error("String contains multiply types")]
    MultiplyTypes,
    #[error("Signature must not exceed the maximum length: {MAXIMUM_SIGNATURE_LENGTH} < {0}")]
    ExceedMaximum(usize),
}

/// Get the char at offset.
#[inline]
fn get_char_at(bytes: &[u8], offset: usize) -> Result<u8, TypeError> {
    match bytes.get(offset) {
        Some(c) => Ok(*c),
        None => Err(TypeError::TooShort(bytes.len(), offset)),
    }
}

/// Get the next type from a `&str`.
fn next_type(
    type_string: &[u8],
    type_string_offset: &mut usize,
    array_depth: u8,
    struct_depth: u8,
    dict_depth: u8,
) -> Result<Type, TypeError> {
    Type::check_depth(array_depth, struct_depth, dict_depth)?;

    let start = *type_string_offset;
    *type_string_offset += 1;
    match get_char_at(type_string, start)? {
        b'y' => Ok(Type::Byte),
        b'b' => Ok(Type::Boolean),
        b'n' => Ok(Type::Int16),
        b'q' => Ok(Type::Uint16),
        b'i' => Ok(Type::Int32),
        b'u' => Ok(Type::Uint32),
        b'x' => Ok(Type::Int64),
        b't' => Ok(Type::Uint64),
        b'd' => Ok(Type::Double),
        b's' => Ok(Type::String),
        b'o' => Ok(Type::ObjectPath),
        b'g' => Ok(Type::Signature),
        #[cfg(target_family = "unix")]
        b'h' => Ok(Type::UnixFD),
        b'v' => Ok(Type::Variant),
        b'a' => {
            let type_ = next_type(
                type_string,
                type_string_offset,
                array_depth + 1,
                struct_depth,
                dict_depth,
            )?;
            let type_ = Box::new(type_);
            Ok(Type::Array(type_))
        }
        b'(' => {
            let first_type = next_type(
                type_string,
                type_string_offset,
                array_depth,
                struct_depth + 1,
                dict_depth,
            )?;
            let mut types = vec![first_type];
            loop {
                if get_char_at(type_string, *type_string_offset)? == b')' {
                    *type_string_offset += 1;
                    return Ok(Type::Struct(types));
                }
                let type_ = next_type(
                    type_string,
                    type_string_offset,
                    array_depth,
                    struct_depth + 1,
                    dict_depth,
                )?;
                types.push(type_);
            }
        }
        b'{' => {
            let key = next_type(
                type_string,
                type_string_offset,
                array_depth,
                struct_depth,
                dict_depth + 1,
            )?;

            let value = next_type(
                type_string,
                type_string_offset,
                array_depth,
                struct_depth,
                dict_depth + 1,
            )?;

            match get_char_at(type_string, *type_string_offset)? {
                b'}' => {
                    *type_string_offset += 1;
                    let type_ = Box::new((key, value));
                    Ok(Type::DictEntry(type_))
                }
                c => Err(TypeError::ClosingCurlyBracket(*type_string_offset, c)),
            }
        }
        c => Err(TypeError::InvalidChar(c)),
    }
}

/// This represents an [interface name].
///
/// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Type {
    Byte,
    Boolean,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    #[cfg(target_family = "unix")]
    UnixFD,
    Double,
    String,
    ObjectPath,
    Variant,
    Signature,
    Array(Box<Type>),
    Struct(Vec<Type>),
    DictEntry(Box<(Type, Type)>),
}

impl Type {
    #[inline]
    pub(crate) const fn check_depth(
        array_depth: u8,
        struct_depth: u8,
        dict_depth: u8,
    ) -> Result<(), TypeError> {
        if MAXIMUM_ARRAY_DEPTH < array_depth {
            return Err(TypeError::ArrayDepth(array_depth));
        }

        if MAXIMUM_STRUCT_DEPTH < struct_depth {
            return Err(TypeError::StructDepth(struct_depth));
        }

        if MAXIMUM_DICT_DEPTH < dict_depth {
            return Err(TypeError::DictDepth(dict_depth));
        }

        Ok(())
    }

    #[inline]
    pub(crate) const fn check_len(signature_string: &str) -> Result<(), TypeError> {
        let signature_string_len = signature_string.len();
        if signature_string_len <= MAXIMUM_SIGNATURE_LENGTH {
            Ok(())
        } else {
            Err(TypeError::ExceedMaximum(signature_string_len))
        }
    }

    pub fn from_string_to_signature(signature_string: &str) -> Result<Vec<Type>, TypeError> {
        Type::from_bytes_to_signature(signature_string.as_bytes())
    }

    pub fn from_bytes_to_signature(signature_string: &[u8]) -> Result<Vec<Type>, TypeError> {
        let signature_string_len = signature_string.len();
        if MAXIMUM_SIGNATURE_LENGTH < signature_string_len {
            return Err(TypeError::ExceedMaximum(signature_string_len));
        }

        let mut signature = Vec::new();
        let mut signature_string_offset = 0;
        while signature_string_offset < signature_string_len {
            let type_ = next_type(signature_string, &mut signature_string_offset, 0, 0, 0)?;
            signature.push(type_);
        }

        Ok(signature)
    }

    pub fn from_signature_to_string(signature: &[Type]) -> Result<String, TypeError> {
        let mut signature_string = String::new();
        for type_ in signature {
            type_.try_to_string(&mut signature_string, 0, 0, 0)?;
        }
        Ok(signature_string)
    }

    pub(crate) fn try_to_string(
        &self,
        type_string: &mut String,
        array_depth: u8,
        struct_depth: u8,
        dict_depth: u8,
    ) -> Result<(), TypeError> {
        Type::check_depth(array_depth, struct_depth, dict_depth)?;
        match self {
            Type::Byte => type_string.push('y'),
            Type::Boolean => type_string.push('b'),
            Type::Int16 => type_string.push('n'),
            Type::Uint16 => type_string.push('q'),
            Type::Int32 => type_string.push('i'),
            Type::Uint32 => type_string.push('u'),
            Type::Int64 => type_string.push('x'),
            Type::Uint64 => type_string.push('t'),
            #[cfg(target_family = "unix")]
            Type::UnixFD => type_string.push('h'),
            Type::Double => type_string.push('d'),
            Type::String => type_string.push('s'),
            Type::ObjectPath => type_string.push('o'),
            Type::Variant => type_string.push('v'),
            Type::Signature => type_string.push('g'),
            Type::Array(type_) => {
                type_string.push('a');
                type_.try_to_string(type_string, array_depth + 1, struct_depth, dict_depth)?;
            }
            Type::Struct(signature) => {
                type_string.push('(');
                for type_ in signature {
                    type_.try_to_string(type_string, array_depth, struct_depth + 1, dict_depth)?;
                }
                type_string.push(')');
            }
            Type::DictEntry(signature) => {
                type_string.push('{');
                signature.0.try_to_string(
                    type_string,
                    array_depth,
                    struct_depth,
                    dict_depth + 1,
                )?;
                signature.1.try_to_string(
                    type_string,
                    array_depth,
                    struct_depth,
                    dict_depth + 1,
                )?;
                type_string.push('}');
            }
        }
        Type::check_len(type_string)?;
        Ok(())
    }

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
            #[cfg(target_family = "unix")]
            Type::UnixFD => 4,
            Type::Double => 8,
            Type::String => 4,
            Type::ObjectPath => 4,
            Type::Variant => 1,
            Type::Signature => 1,
            Type::Array(_) => 4,
            Type::Struct(_) => 8,
            Type::DictEntry(_) => 8,
        }
    }
}

impl TryFrom<&str> for Type {
    type Error = TypeError;

    fn try_from(type_string: &str) -> Result<Self, Self::Error> {
        let mut type_string_offset = 0;
        let type_ = next_type(type_string.as_bytes(), &mut type_string_offset, 0, 0, 0)?;
        if type_string.len() == type_string_offset {
            Ok(type_)
        } else {
            Err(TypeError::MultiplyTypes)
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Type::Byte => write!(f, "y"),
            Type::Boolean => write!(f, "b"),
            Type::Int16 => write!(f, "n"),
            Type::Uint16 => write!(f, "q"),
            Type::Int32 => write!(f, "i"),
            Type::Uint32 => write!(f, "u"),
            Type::Int64 => write!(f, "x"),
            Type::Uint64 => write!(f, "t"),
            #[cfg(target_family = "unix")]
            Type::UnixFD => write!(f, "h"),
            Type::Double => write!(f, "d"),
            Type::String => write!(f, "s"),
            Type::ObjectPath => write!(f, "o"),
            Type::Variant => write!(f, "v"),
            Type::Signature => write!(f, "g"),
            Type::Array(type_) => write!(f, "a{}", type_),
            Type::Struct(types) => {
                write!(f, "(")?;
                for type_ in types {
                    write!(f, "{}", type_)?;
                }
                write!(f, ")")
            }
            Type::DictEntry(dict_entry) => write!(f, "{{{}{}}}", dict_entry.0, dict_entry.1),
        }
    }
}
