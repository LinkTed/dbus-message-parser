use crate::{DecodeResult, Decoder, Value};

mod basic_type;

mod container;

mod value;

#[cfg(test)]
mod tests;

impl<'a> Decoder<'a> {
    /// Decode a byte array to a `Value` object.
    pub fn value(&mut self, is_le: bool, sig: &str) -> DecodeResult<Vec<Value>> {
        self.v(is_le, 0, 0, sig)
    }
}
