mod basic_type;
mod container;
mod value;

use crate::{DecodeResult, Decoder, Value};
use bytes::Buf;
use std::ops::Deref;

impl<'a, T> Decoder<'a, T>
where
    T: Buf + Deref<Target = [u8]>,
{
    /// Decode a byte array to a `Value` object.
    pub fn value(&mut self, is_le: bool, sig: &str) -> DecodeResult<Vec<Value>> {
        self.v(is_le, 0, 0, sig)
    }
}
