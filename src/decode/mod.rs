mod decoder;
mod error;
mod message;
#[cfg(test)]
mod tests;
mod value;

pub(crate) use decoder::Decoder;
pub use error::{DecodeError, DecodeResult};

pub const MAXIMUM_VARIANT_DEPTH: u8 = 4;
