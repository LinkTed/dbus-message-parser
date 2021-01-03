use crate::decode::{DecodeError, DecodeResult, Decoder};
use crate::value::{Signature, Type, Value};

impl<'a> Decoder<'a> {
    /// Decode a byte array to a `Value` object.
    pub(crate) fn value(
        &mut self,
        is_le: bool,
        variant_depth: u8,
        signature: &Signature,
    ) -> DecodeResult<Vec<Value>> {
        let mut r = Vec::new();
        // Decode the value according to the signature.
        for s in signature.iter() {
            match s.get_type() {
                Some(t) => {
                    let v = match t {
                        Type::Byte => self.byte()?,
                        Type::Boolean => self.boolean(is_le)?,
                        Type::Int16 => self.int_16(is_le)?,
                        Type::Uint16 => self.uint_16(is_le)?,
                        Type::Int32 => self.int_32(is_le)?,
                        Type::Uint32 => self.uint_32(is_le)?,
                        Type::Int64 => self.int_64(is_le)?,
                        Type::Uint64 => self.uint_64(is_le)?,
                        Type::Double => self.double(is_le)?,
                        Type::String => self.string(is_le)?,
                        Type::ObjectPath => self.object_path(is_le)?,
                        Type::Signature => self.signature()?,
                        #[cfg(target_family = "unix")]
                        Type::UnixFD => self.unix_fd(is_le)?,
                        Type::Array(signature) => self.array(is_le, variant_depth, &signature)?,
                        Type::Struct(signature) => {
                            self.decode_struct(is_le, variant_depth, &signature)?
                        }
                        Type::DictEntry(signature_key, signature_value) => {
                            self.dict_entry(is_le, variant_depth, &signature_key, &signature_value)?
                        }
                        Type::Variant => self.variant(is_le, variant_depth)?,
                    };

                    r.push(v);
                }
                None => return Err(DecodeError::SignatureEmpty),
            }
        }

        Ok(r)
    }
}
