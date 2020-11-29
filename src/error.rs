use crate::{
    BusError, ErrorError, InterfaceError, MemberError, MessageHeaderError, ObjectPathError, Value,
};
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
    BusError(BusError),
    ObjectPathError(ObjectPathError),
    InterfaceError(InterfaceError),
    MemberError(MemberError),
    ErrorError(ErrorError),
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
    MessageHeaderError(MessageHeaderError),
    IntegerOverflow(usize, usize),
}

impl From<FromUtf8Error> for DecodeError {
    fn from(e: FromUtf8Error) -> Self {
        DecodeError::Utf8Error(e)
    }
}

impl From<BusError> for DecodeError {
    fn from(e: BusError) -> Self {
        DecodeError::BusError(e)
    }
}

impl From<ObjectPathError> for DecodeError {
    fn from(e: ObjectPathError) -> Self {
        DecodeError::ObjectPathError(e)
    }
}

impl From<InterfaceError> for DecodeError {
    fn from(e: InterfaceError) -> Self {
        DecodeError::InterfaceError(e)
    }
}

impl From<MemberError> for DecodeError {
    fn from(e: MemberError) -> Self {
        DecodeError::MemberError(e)
    }
}

impl From<ErrorError> for DecodeError {
    fn from(e: ErrorError) -> Self {
        DecodeError::ErrorError(e)
    }
}

impl From<MessageHeaderError> for DecodeError {
    fn from(e: MessageHeaderError) -> Self {
        DecodeError::MessageHeaderError(e)
    }
}
