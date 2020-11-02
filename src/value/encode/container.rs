use crate::{EncodeError, EncodeResult, Encoder, Value};

impl<'a> Encoder<'a> {
    /// Encode a `&Vec<Value>` as an array into the buffer.
    pub fn array(&mut self, vec: &[Value], sig: &str, is_le: bool) -> EncodeResult {
        let array_len_offset = self.buf.len();
        self.uint_32(0, is_le);

        match sig.get(0..1) {
            Some(s) => match s {
                "v" | "y" | "g" => {}
                "n" | "q" => self.algin(2),
                #[cfg(target_family = "unix")]
                "h" => self.algin(4),
                "b" | "i" | "u" | "a" | "s" | "o" => self.algin(4),
                "x" | "t" | "d" | "(" | "{" => self.algin(8),
                signature => return Err(EncodeError::UnknownSignature(signature.to_string())),
            },
            None => return Err(EncodeError::NullSignature),
        }

        let array_len_offset_algin = self.buf.len();

        let mut sig_cmp = String::new();
        for v in vec {
            v.get_signature(&mut sig_cmp);
            if sig == sig_cmp {
                sig_cmp.clear();
            } else {
                return Err(EncodeError::ArraySignatureMismatch(
                    sig.to_string(),
                    sig_cmp,
                ));
            }
            self.value(v, is_le)?;
        }

        let array_len = (self.buf.len() - array_len_offset_algin) as u32;
        self.set_uint_32(array_len, array_len_offset, is_le);

        Ok(())
    }

    /// Encode a `&[Value]` as a struct into the buffer.
    pub fn encode_struct(&mut self, values: &[Value], is_le: bool) -> EncodeResult {
        for v in values {
            self.value(v, is_le)?;
        }

        Ok(())
    }

    /// Encode a `&(Value, Value)` as a dict entry into the buffer.
    pub fn dict_entry(&mut self, b: &(Value, Value), is_le: bool) -> EncodeResult {
        let (key, value) = &*b;
        self.value(key, is_le)?;
        self.value(value, is_le)
    }

    /// Encode a `&[Value]` as a variant into the buffer.
    pub fn variant(&mut self, variant: &Value, is_le: bool) -> EncodeResult {
        let mut sig = String::new();
        variant.get_signature(&mut sig);
        self.signature(&sig)?;

        self.value(variant, is_le)?;

        Ok(())
    }
}
