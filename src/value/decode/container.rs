use crate::{DecodeError, DecodeResult, Decoder, Value};
use std::ops::Deref;

impl<'a, T> Decoder<'a, T>
where
    T: Deref<Target = [u8]>,
{
    /// Decode from a byte array at a specific offset to a `Value::Variant`.
    pub fn variant(&mut self, is_le: bool) -> DecodeResult<Value> {
        let signature = self.sig()?;
        let mut v = self.value(is_le, &signature)?;
        if v.len() == 1 {
            let v = v.pop().unwrap();
            Ok(Value::Variant(Box::new(v)))
        } else {
            Err(DecodeError::VariantError(v))
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::Array`.
    pub fn array(
        &mut self,
        is_le: bool,
        mut array_recursion: u8,
        struct_recursion: u8,
        signature: &str,
    ) -> DecodeResult<Value> {
        array_recursion += 1;

        let array_size = self.u_32(is_le)?;

        if 67108864 < array_size {
            return Err(DecodeError::ArrayTooBig);
        }

        match signature.get(0..1) {
            Some(s) => match s {
                "v" | "y" | "g" => {}
                "n" | "q" => self.algin(2)?,
                "b" | "i" | "u" | "a" | "s" | "o" => self.algin(4)?,
                "x" | "t" | "(" | "{" => self.algin(8)?,
                _ => return Err(DecodeError::Signature),
            },
            None => return Err(DecodeError::Signature),
        };

        let mut r = Vec::new();

        let end = Decoder::<'a, T>::checked_add(self.offset, array_size as usize)?;
        while self.offset < end {
            let mut v = self.v(is_le, array_recursion, struct_recursion, signature)?;
            if v.len() == 1 {
                r.push(v.pop().unwrap());
            } else {
                return Err(DecodeError::ArrayVecLen);
            }
        }

        if self.offset == end {
            Ok(Value::Array(r, String::from(signature)))
        } else {
            Err(DecodeError::ArrayLen)
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::Struct`.
    pub fn decode_struct(
        &mut self,
        is_le: bool,
        array_recursion: u8,
        mut struct_recursion: u8,
        signature: &str,
    ) -> DecodeResult<Value> {
        struct_recursion += 1;
        let v = self.v(is_le, array_recursion, struct_recursion, signature)?;
        Ok(Value::Struct(v))
    }

    /// Decode from a byte array at a specific offset to a `Value::DictEntry`.
    pub fn dict_entry(
        &mut self,
        is_le: bool,
        array_recursion: u8,
        struct_recursion: u8,
        signature_key: &str,
        signature_value: &str,
    ) -> DecodeResult<Value> {
        let mut v = self.v(is_le, array_recursion, struct_recursion, signature_key)?;
        let k = if v.len() == 1 {
            v.pop().unwrap()
        } else {
            return Err(DecodeError::ArrayVecLen);
        };

        let mut v = self.v(is_le, array_recursion, struct_recursion, signature_value)?;
        let v = if v.len() == 1 {
            v.pop().unwrap()
        } else {
            return Err(DecodeError::ArrayVecLen);
        };

        Ok(Value::DictEntry(Box::new((k, v))))
    }
}
