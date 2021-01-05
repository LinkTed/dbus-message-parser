use crate::value::{Type, Value, MAXIMUM_ARRAY_DEPTH, MAXIMUM_DICT_DEPTH, MAXIMUM_STRUCT_DEPTH};
use std::cmp::{Eq, PartialEq};
use std::convert::{AsRef, From, TryFrom, TryInto};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

pub const MAXIMUM_SIGNATURE_LENGTH: usize = 255;

/// An enum representing all errors, which can occur during the handling of a [`Signature`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum SignatureError {
    #[error("Signature contians a invalid: {0}")]
    InvalidChar(u8),
    #[error("Array depth is too big: {MAXIMUM_ARRAY_DEPTH} < {0}")]
    ArrayDepth(u8),
    #[error("Struct depth is too big: {MAXIMUM_STRUCT_DEPTH} < {0}")]
    StructDepth(u8),
    #[error("Dict depth is too big: {MAXIMUM_DICT_DEPTH} < {0}")]
    DictDepth(u8),
    #[error("Signature is too big: {MAXIMUM_SIGNATURE_LENGTH} < {0}")]
    TooBig(usize),
    #[error("Signature is too short: got {0} offset {1}")]
    TooShort(usize, usize),
    #[error("The closing curly bracket is missing for the dict at {0} got {1}")]
    ClosingCurlyBracket(usize, u8),
}

/// Check if the string is a valid signature.
fn check_signature(signature: &str) -> Result<(), SignatureError> {
    let mut signature_offset = 0;

    let signature_len = signature.len();
    if MAXIMUM_SIGNATURE_LENGTH < signature_len {
        return Err(SignatureError::TooBig(signature_len));
    }

    while signature_offset < signature_len {
        get_next_signature(signature.as_bytes(), &mut signature_offset, 0, 0, 0)?;
    }

    Ok(())
}

/// Get the char at offset.
#[inline]
fn get_char_at(signature: &[u8], offset: usize) -> Result<u8, SignatureError> {
    match signature.get(offset) {
        Some(c) => Ok(*c),
        None => Err(SignatureError::TooShort(signature.len(), offset)),
    }
}

/// Get the next signature from a `&str`.
fn get_next_signature<'a>(
    signature: &'a [u8],
    signature_offset: &mut usize,
    array_depth: u8,
    struct_depth: u8,
    dict_depth: u8,
) -> Result<&'a [u8], SignatureError> {
    if MAXIMUM_ARRAY_DEPTH < array_depth {
        return Err(SignatureError::ArrayDepth(array_depth));
    }

    if MAXIMUM_STRUCT_DEPTH < struct_depth {
        return Err(SignatureError::StructDepth(struct_depth));
    }

    if MAXIMUM_DICT_DEPTH < dict_depth {
        return Err(SignatureError::DictDepth(dict_depth));
    }

    let start = *signature_offset;
    *signature_offset += 1;
    match get_char_at(signature, start)? {
        b'y' | b'b' | b'n' | b'q' | b'i' | b'u' | b'x' | b't' | b'd' | b's' | b'o' | b'g'
        | b'v' => Ok(&signature[start..*signature_offset]),
        #[cfg(target_family = "unix")]
        b'h' => Ok(&signature[start..*signature_offset]),
        b'a' => {
            get_next_signature(
                signature,
                signature_offset,
                array_depth + 1,
                struct_depth,
                dict_depth,
            )?;
            Ok(&signature[start..*signature_offset])
        }
        b'(' => {
            get_next_signature(
                signature,
                signature_offset,
                array_depth,
                struct_depth + 1,
                dict_depth,
            )?;
            loop {
                if get_char_at(signature, *signature_offset)? == b')' {
                    *signature_offset += 1;
                    return Ok(&signature[start..*signature_offset]);
                }
                get_next_signature(
                    signature,
                    signature_offset,
                    array_depth,
                    struct_depth + 1,
                    dict_depth,
                )?;
            }
        }
        b'{' => {
            get_next_signature(
                signature,
                signature_offset,
                array_depth,
                struct_depth,
                dict_depth + 1,
            )?;

            get_next_signature(
                signature,
                signature_offset,
                array_depth,
                struct_depth,
                dict_depth + 1,
            )?;

            match get_char_at(signature, *signature_offset)? {
                b'}' => {
                    *signature_offset += 1;
                    Ok(&signature[start..*signature_offset])
                }
                c => Err(SignatureError::ClosingCurlyBracket(*signature_offset, c)),
            }
        }
        c => Err(SignatureError::InvalidChar(c)),
    }
}

/// This represents an [interface name].
///
/// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Signature(String);

impl Signature {
    pub fn iter(&self) -> SignatureIter {
        SignatureIter {
            signature: self,
            signature_offset: 0,
        }
    }

    /// Get the type of the first value of the signature, if the signature is not empty.
    pub fn get_type(&self) -> Option<Type> {
        let sig = self.as_ref();
        if sig.is_empty() {
            return None;
        }
        let t = match get_char_at(sig.as_bytes(), 0).unwrap() {
            b'y' => Type::Byte,
            b'b' => Type::Boolean,
            b'n' => Type::Int16,
            b'q' => Type::Uint16,
            b'i' => Type::Int32,
            b'u' => Type::Uint32,
            b'x' => Type::Int64,
            b't' => Type::Uint64,
            b'd' => Type::Double,
            b's' => Type::String,
            b'o' => Type::ObjectPath,
            b'g' => Type::Signature,
            #[cfg(target_family = "unix")]
            b'h' => Type::UnixFD,
            b'a' => {
                let mut signature_offset = 1;
                let sig =
                    get_next_signature(sig.as_bytes(), &mut signature_offset, 0, 0, 0).unwrap();
                let sig = String::from_utf8(sig.to_owned()).unwrap();
                Type::Array(Signature(sig))
            }
            b'(' => {
                let mut signature_offset = 1;
                get_next_signature(sig.as_bytes(), &mut signature_offset, 0, 0, 0).unwrap();
                while get_char_at(sig.as_bytes(), signature_offset).unwrap() != b')' {
                    get_next_signature(sig.as_bytes(), &mut signature_offset, 0, 0, 0).unwrap();
                }
                Type::Struct(Signature(sig[1..signature_offset].to_owned()))
            }
            b'{' => {
                let mut signature_offset = 1;
                let key =
                    get_next_signature(sig.as_bytes(), &mut signature_offset, 0, 0, 0).unwrap();
                let key = String::from_utf8(key.to_vec()).unwrap();
                let key = Signature(key);
                let value =
                    get_next_signature(sig.as_bytes(), &mut signature_offset, 0, 0, 0).unwrap();
                let value = String::from_utf8(value.to_owned()).unwrap();
                let value = Signature(value);
                Type::DictEntry(key, value)
            }
            b'v' => Type::Variant,
            c => panic!("Invalid char: {}", c),
        };
        Some(t)
    }

    pub fn new(values: &[Value]) -> Result<Signature, SignatureError> {
        let mut signature = String::new();
        for value in values {
            value.get_signature_as_string(&mut signature);
        }
        signature.try_into()
    }
}

impl From<Signature> for String {
    fn from(signature: Signature) -> Self {
        signature.0
    }
}

impl TryFrom<String> for Signature {
    type Error = SignatureError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        check_signature(&value)?;
        let signature = Signature(value);
        Ok(signature)
    }
}

impl TryFrom<&str> for Signature {
    type Error = SignatureError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        check_signature(&value)?;
        let signature = Signature(value.to_owned());
        Ok(signature)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Signature {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Signature {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

pub struct SignatureIter<'a> {
    signature: &'a Signature,
    signature_offset: usize,
}

impl<'a> Iterator for SignatureIter<'a> {
    type Item = Signature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.signature_offset == self.signature.as_ref().len() {
            return None;
        }

        let signature = get_next_signature(
            self.signature.0.as_bytes(),
            &mut self.signature_offset,
            0,
            0,
            0,
        )
        .unwrap();
        let signature = String::from_utf8(signature.to_owned()).unwrap();
        let signature = Signature(signature);
        Some(signature)
    }
}
