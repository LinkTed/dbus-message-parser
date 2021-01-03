use crate::encode::{EncodeError, EncodeResult, Encoder};
use crate::value::{Signature, Value, MAXIMUM_ARRAY_LENGTH};

impl Encoder {
    /// Apply the alignment and encode a `&[Value]` as an array into the buffer.
    pub fn array(&mut self, vec: &[Value], signature: &Signature, is_le: bool) -> EncodeResult<()> {
        self.algin(4);
        let array_len_offset = self.buf.len();
        self.u_32(0, is_le);

        match signature.get_type() {
            Some(t) => self.algin(t.get_alignment()),
            None => return Err(EncodeError::ArraySignatureEmpty),
        }

        let array_len_offset_algin = self.buf.len();

        for v in vec {
            let signature_v = v.get_signature()?;
            if signature != &signature_v {
                return Err(EncodeError::ArraySignatureMismatch(
                    signature.to_owned(),
                    signature_v,
                ));
            }
            self.value(v, is_le)?;
        }

        let array_len = self.buf.len() - array_len_offset_algin;
        if MAXIMUM_ARRAY_LENGTH < array_len {
            return Err(EncodeError::ArrayTooBig(array_len));
        }
        let array_len = array_len as u32;
        self.set_uint_32(array_len, array_len_offset, is_le);

        Ok(())
    }

    /// Apply the alignment and encode a `&[Value]` as a struct into the buffer.
    pub fn encode_struct(&mut self, values: &[Value], is_le: bool) -> EncodeResult<()> {
        self.algin(8);
        for v in values {
            self.value(v, is_le)?;
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
        let signature = variant.get_signature()?;
        self.signature(&signature)?;
        self.value(variant, is_le)
    }
}
