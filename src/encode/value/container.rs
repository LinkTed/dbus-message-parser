use crate::encode::{EncodeError, EncodeResult, Encoder};
use crate::value::{Array, Struct, Value, MAXIMUM_ARRAY_LENGTH};
use std::slice::from_ref;

impl Encoder {
    /// Apply the alignment and encode a `&Array` as an array into the buffer.
    pub fn array(&mut self, array: &Array, is_le: bool) -> EncodeResult<()> {
        self.algin(4);
        let array_len_offset = self.buf.len();
        self.u_32(0, is_le);

        self.algin(array.get_type().get_alignment());
        let array_len_offset_algin = self.buf.len();

        for value in array.as_ref() {
            self.value(value, is_le)?;
        }

        let array_len = self.buf.len() - array_len_offset_algin;
        if MAXIMUM_ARRAY_LENGTH < array_len {
            return Err(EncodeError::ArrayTooBig(array_len));
        }
        let array_len = array_len as u32;
        self.set_uint_32(array_len, array_len_offset, is_le);

        Ok(())
    }

    /// Apply the alignment and encode a `&Struct` as a struct into the buffer.
    pub fn encode_struct(&mut self, struct_: &Struct, is_le: bool) -> EncodeResult<()> {
        self.algin(8);
        for value in struct_.as_ref() {
            self.value(value, is_le)?;
        }

        Ok(())
    }

    /// Apply the alignment and encode a `&(Value, Value)` as a dict entry into the buffer.
    pub fn dict_entry(&mut self, b: &(Value, Value), is_le: bool) -> EncodeResult<()> {
        self.algin(8);
        let (key, value) = &*b;
        self.value(key, is_le)?;
        self.value(value, is_le)
    }

    /// Encode a `&[Value]` as a variant into the buffer.
    pub fn variant(&mut self, variant: &Value, is_le: bool) -> EncodeResult<()> {
        let type_ = variant.get_type()?;
        let signature = from_ref(&type_);
        self.signature(signature)?;
        self.value(variant, is_le)
    }
}
