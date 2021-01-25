use crate::decode::{DecodeResult, Decoder};
use crate::value::{Type, Value};

impl<'a> Decoder<'a> {
    /// Decode a byte array to a `Vec<Value>` object.
    pub(crate) fn values(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature: &[Type],
    ) -> DecodeResult<Vec<Value>> {
        let mut result = Vec::with_capacity(signature.len());
        // Decode the value according to the signature.
        for type_ in signature {
            let value = self.value(is_le, variant_depth, type_)?;
            result.push(value);
        }

        Ok(result)
    }

    /// Decode a byte array to a `Value` object.
    pub(crate) fn value(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        type_: &Type,
    ) -> DecodeResult<Value> {
        match type_ {
            Type::Byte => self.byte(),
            Type::Boolean => self.boolean(is_le),
            Type::Int16 => self.int_16(is_le),
            Type::Uint16 => self.uint_16(is_le),
            Type::Int32 => self.int_32(is_le),
            Type::Uint32 => self.uint_32(is_le),
            Type::Int64 => self.int_64(is_le),
            Type::Uint64 => self.uint_64(is_le),
            Type::Double => self.double(is_le),
            Type::String => self.string(is_le),
            Type::ObjectPath => self.object_path(is_le),
            Type::Signature => self.signature(),
            #[cfg(target_family = "unix")]
            Type::UnixFD => self.unix_fd(is_le),
            Type::Array(type_) => self.array(is_le, variant_depth, &type_),
            Type::Struct(signature) => self.decode_struct(is_le, variant_depth, &signature),
            Type::DictEntry(signature) => {
                self.dict_entry(is_le, variant_depth, &signature.0, &signature.1)
            }
            Type::Variant => self.variant(is_le, variant_depth),
        }
    }
}
