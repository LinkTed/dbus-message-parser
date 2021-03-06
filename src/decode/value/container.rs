use crate::decode::{DecodeError, DecodeResult, Decoder, MAXIMUM_VARIANT_DEPTH};
use crate::value::{Array, Struct, Type, Value, MAXIMUM_ARRAY_LENGTH};
use std::convert::TryFrom;

impl<'a> Decoder<'a> {
    /// Decode from a byte array at a specific offset to a [`Value::Variant`].
    ///
    /// [`Value::Variant`]: crate::value::Value::Variant
    pub fn variant(&mut self, is_le: bool, mut variant_depth: u8) -> DecodeResult<Value> {
        variant_depth += 1;
        if MAXIMUM_VARIANT_DEPTH < variant_depth {
            return Err(DecodeError::VariantDepth(variant_depth));
        }

        let type_ = self.d_type()?;
        let value = self.value(is_le, variant_depth, &type_)?;
        Ok(Value::Variant(Box::new(value)))
    }

    /// Check alignment and decode from a byte array at a specific offset to a `Vec<Value>`.
    pub(crate) fn d_array(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        type_: &Type,
    ) -> DecodeResult<Vec<Value>> {
        let array_size = self.u_32(is_le)?;
        if MAXIMUM_ARRAY_LENGTH < array_size as usize {
            return Err(DecodeError::ArrayTooBig(array_size));
        }

        self.algin(type_.get_alignment())?;
        let mut array = Vec::new();
        let end = Decoder::<'a>::checked_add(self.offset, array_size as usize)?;
        while self.offset < end {
            let value = self.value(is_le, variant_depth, type_)?;
            array.push(value);
        }

        if self.offset == end {
            Ok(array)
        } else {
            Err(DecodeError::ArrayInvalidLength(self.offset, end))
        }
    }

    /// Decode from a byte array at a specific offset to a [`Value::Array`].
    ///
    /// [`Value::Array`]: crate::value::Value::Array
    pub fn array(&mut self, is_le: bool, variant_depth: u8, type_: &Type) -> DecodeResult<Value> {
        let array = self.d_array(is_le, variant_depth, type_)?;
        let array = Array {
            array,
            type_: type_.clone(),
        };
        Ok(Value::Array(array))
    }

    /// Decode from a byte array at a specific offset to a [`Value::Struct`].
    ///
    /// [`Value::Struct`]: crate::value::Value::Struct
    pub fn decode_struct(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature: &[Type],
    ) -> DecodeResult<Value> {
        self.algin(8)?;
        let values = self.values(is_le, variant_depth, signature)?;
        let struct_ = Struct::try_from(values)?;
        Ok(Value::Struct(struct_))
    }

    /// Decode from a byte array at a specific offset to a [`Value::DictEntry`].
    ///
    /// [`Value::DictEntry`]: crate::value::Value::DictEntry
    pub fn dict_entry(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        key_type: &Type,
        value_type: &Type,
    ) -> DecodeResult<Value> {
        self.algin(8)?;
        let key = self.value(is_le, variant_depth, key_type)?;
        let value = self.value(is_le, variant_depth, value_type)?;
        Ok(Value::DictEntry(Box::new((key, value))))
    }
}
