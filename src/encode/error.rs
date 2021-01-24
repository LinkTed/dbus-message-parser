use crate::value::{Type, TypeError, MAXIMUM_ARRAY_LENGTH};
use thiserror::Error;

pub type EncodeResult<T> = Result<T, EncodeError>;

/// An enum representing all errors, which can occur during the encoding.
#[derive(Debug, PartialEq, Error)]
pub enum EncodeError {
    #[error("Array length is too big: {MAXIMUM_ARRAY_LENGTH} < {0}")]
    ArrayTooBig(usize),
    #[error("The body length is zero, but there is a body signature '{0:?}'")]
    BodyLengthZero(Vec<Type>),
    #[error("The body signature is missing, but there body length 0 != {0}")]
    BodySignatureMissing(u32),
    #[error("Could not encode Signature: {0}")]
    SignatureError(#[from] TypeError),
}
