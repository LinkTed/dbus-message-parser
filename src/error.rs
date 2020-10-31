use crate::{Value, ObjectPathError};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

pub type EncodeResult = Result<(), EncodeError>;

/// An enum representing all errors, which can occur during the encoding.
#[derive(Debug, PartialEq)]
pub enum EncodeError {
    ArraySignatureMismatch(String, String),
    UnknownSignature(String),
    NullSignature,
    SignatureTooLarge(TryFromIntError),
}

impl From<TryFromIntError> for EncodeError {
    fn from(e: TryFromIntError) -> Self {
        EncodeError::SignatureTooLarge(e)
    }
}

pub type DecodeResult<T> = Result<T, DecodeError>;

/// An enum representing all errors, which can occur during the decoding.
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    TooShort,
    VariantError(Vec<Value>),
    InvalidBoolean(u32),
    Utf8Error(FromUtf8Error),
    StringNotNull,
    ObjectPathError(ObjectPathError),
    InterfaceRegex,
    MemberRegex,
    BusNamesRegex,
    Signature,
    SignatureTooBig,
    Padding,
    ArrayTooBig,
    ArrayVecLen,
    ArrayLen,
    Endianness,
    Error,
    Header,
    MessageType,
    MessageFlags,
    BodySignatureMissing,
    DictVecLen,
    ArrayRecursion,
    StructRecursion,
    OutOfBoundsFds,
    BodyLength(usize, usize),
}

impl From<FromUtf8Error> for DecodeError {
    fn from(e: FromUtf8Error) -> Self {
        DecodeError::Utf8Error(e)
    }
}

impl From<ObjectPathError> for DecodeError {
    fn from(e: ObjectPathError) -> Self {
        DecodeError::ObjectPathError(e)
    }
}
