use crate::decode::{DecodeError, DecodeResult, Decoder, MAXIMUM_VARIANT_DEPTH};
use crate::value::{Signature, Value, MAXIMUM_ARRAY_LENGTH};

impl<'a> Decoder<'a> {
    /// Decode from a byte array at a specific offset to a [`Value::Variant`].
    ///
    /// [`Value::Variant`]: crate::value::Value::Variant
    pub fn variant(&mut self, is_le: bool, mut variant_depth: u8) -> DecodeResult<Value> {
        variant_depth += 1;
        if MAXIMUM_VARIANT_DEPTH < variant_depth {
            return Err(DecodeError::VariantDepth(variant_depth));
        }

        let signature = self.d_signature()?;
        let mut v = self.value(is_le, variant_depth, &signature)?;
        if v.len() == 1 {
            let v = v.pop().unwrap();
            Ok(Value::Variant(Box::new(v)))
        } else {
            Err(DecodeError::VariantSingleValue(v))
        }
    }

    /// Check alignment and decode from a byte array at a specific offset to a `Vec<Value>`.
    pub(crate) fn d_array(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature: &Signature,
    ) -> DecodeResult<Vec<Value>> {
        let array_size = self.u_32(is_le)?;
        if MAXIMUM_ARRAY_LENGTH < array_size as usize {
            return Err(DecodeError::ArrayTooBig(array_size));
        }

        match signature.get_type() {
            Some(t) => self.algin(t.get_alignment())?,
            None => return Err(DecodeError::ArraySignatureEmpty),
        }

        let mut r = Vec::new();

        let end = Decoder::<'a>::checked_add(self.offset, array_size as usize)?;
        while self.offset < end {
            let mut v = self.value(is_le, variant_depth, signature)?;
            if v.len() == 1 {
                r.push(v.pop().unwrap());
            } else {
                return Err(DecodeError::ArraySingleValue(v));
            }
        }

        if self.offset == end {
            Ok(r)
        } else {
            Err(DecodeError::ArrayInvalidLength(self.offset, end))
        }
    }

    /// Decode from a byte array at a specific offset to a [`Value::Array`].
    ///
    /// [`Value::Array`]: crate::value::Value::Array
    pub fn array(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature: &Signature,
    ) -> DecodeResult<Value> {
        let vec = self.d_array(is_le, variant_depth, signature)?;
        Ok(Value::Array(vec, signature.to_owned()))
    }

    /// Decode from a byte array at a specific offset to a [`Value::Struct`].
    ///
    /// [`Value::Struct`]: crate::value::Value::Struct
    pub fn decode_struct(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature: &Signature,
    ) -> DecodeResult<Value> {
        self.algin(8)?;
        let v = self.value(is_le, variant_depth, signature)?;
        Ok(Value::Struct(v))
    }

    /// Decode from a byte array at a specific offset to a [`Value::DictEntry`].
    ///
    /// [`Value::DictEntry`]: crate::value::Value::DictEntry
    pub fn dict_entry(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature_key: &Signature,
        signature_value: &Signature,
    ) -> DecodeResult<Value> {
        self.algin(8)?;
        let mut v = self.value(is_le, variant_depth, signature_key)?;
        let k = if v.len() == 1 {
            v.pop().unwrap()
        } else {
            return Err(DecodeError::DictKeySingleValue(v));
        };

        let mut v = self.value(is_le, variant_depth, signature_value)?;
        let v = if v.len() == 1 {
            v.pop().unwrap()
        } else {
            return Err(DecodeError::DictValueSingleValue(v));
        };

        Ok(Value::DictEntry(Box::new((k, v))))
    }
}
