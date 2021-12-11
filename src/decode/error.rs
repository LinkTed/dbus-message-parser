use crate::{
    decode::MAXIMUM_VARIANT_DEPTH,
    message::{MessageHeaderError, MessageHeaderFieldsError},
    value::{
        BusError, ErrorError, InterfaceError, MemberError, ObjectPathError, StructError, Type,
        TypeError, MAXIMUM_ARRAY_LENGTH,
    },
};
use std::str::Utf8Error;
use thiserror::Error;

pub type DecodeResult<T> = Result<T, DecodeError>;

/// An enum representing all errors, which can occur during the decoding.
#[derive(Debug, PartialEq, Error)]
pub enum DecodeError {
    #[error("Not enough bytes to decode: got {0} offset {1}")]
    NotEnoughBytes(usize, usize),
    #[error("Boolean value only can be 0 or 1: {0}")]
    InvalidBoolean(u32),
    #[error("Could not decode string as UTF-8: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Last byte is not null: {0}")]
    StringNotNull(u8),
    #[error("Could not decode Bus: {0}")]
    BusError(#[from] BusError),
    #[error("Could not decode ObjectPath: {0}")]
    ObjectPathError(#[from] ObjectPathError),
    #[error("Could not decode Interface: {0}")]
    InterfaceError(#[from] InterfaceError),
    #[error("Could not decode Member: {0}")]
    MemberError(#[from] MemberError),
    #[error("Could not decode Error: {0}")]
    ErrorError(#[from] ErrorError),
    #[error("Could not decode Signature: {0}")]
    SignatureError(#[from] TypeError),
    #[error("Could not decode Struct: {0}")]
    StructError(#[from] StructError),
    #[error("Padding is not zero: {0}")]
    Padding(u8),
    #[error("Array length is too big: {MAXIMUM_ARRAY_LENGTH} < {0}")]
    ArrayTooBig(u32),
    #[error("Array is invalid: got {0} excepted {1}")]
    ArrayInvalidLength(usize, usize),
    #[error("Could not decode the endianness: {0}")]
    Endianness(u8),
    #[error("Could not decode MessageType: {0}")]
    MessageType(u8),
    #[error("Could not decode MessageFlags: {0}")]
    MessageFlags(u8),
    #[error("The body length is zero, but there is a body signature '{0:?}'")]
    BodyLengthZero(Vec<Type>),
    #[error("The body signature is missing, but there body length 0 != {0}")]
    BodySignatureMissing(u32),
    #[error("Not enough FDs: got {0} offset {1}")]
    NotEnoughFds(usize, usize),
    #[error("Could not the body: expected {0} got {1}")]
    BodyLength(usize, usize),
    #[error("Could not decode MessageHeader: {0}")]
    MessageHeaderError(#[from] MessageHeaderError),
    #[error("Could not decode MessageHeaderFields: {0}")]
    MessageHeaderFieldsError(#[from] MessageHeaderFieldsError),
    #[error("Integer overflows occours: {0} + {1}")]
    IntegerOverflow(usize, usize),
    #[error("Variant depth is too big: {MAXIMUM_VARIANT_DEPTH} < {0}")]
    VariantDepth(u8),
}
