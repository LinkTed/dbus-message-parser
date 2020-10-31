mod basic_type;
mod container;

use super::Value;
use crate::{EncodeResult, Encoder};

impl<'a> Encoder<'a> {
    /// Encode a `Value` object to a byte array.
    pub fn value(&mut self, value: &Value, is_le: bool) -> EncodeResult {
        match value {
            Value::Byte(b) => {
                self.byte(*b);
                Ok(())
            }
            Value::Boolean(b) => {
                self.algin(4);
                self.boolean(*b, is_le);
                Ok(())
            }
            Value::Int16(i) => {
                self.algin(2);
                self.int_16(*i, is_le);
                Ok(())
            }
            Value::Uint16(u) => {
                self.algin(2);
                self.uint_16(*u, is_le);
                Ok(())
            }
            Value::Int32(i) => {
                self.algin(4);
                self.int_32(*i, is_le);
                Ok(())
            }
            Value::Uint32(u) => {
                self.algin(4);
                self.uint_32(*u, is_le);
                Ok(())
            }
            Value::Int64(i) => {
                self.algin(8);
                self.int_64(*i, is_le);
                Ok(())
            }
            Value::Uint64(u) => {
                self.algin(8);
                self.uint_64(*u, is_le);
                Ok(())
            }
            Value::Double(f) => {
                self.algin(8);
                self.double(*f, is_le);
                Ok(())
            }
            Value::ObjectPath(s) => {
                self.algin(4);
                self.path(s, is_le)
            }
            Value::String(s) => {
                self.algin(4);
                self.string(s, is_le);
                Ok(())
            }
            Value::Signature(s) => self.signature(s),
            Value::Array(vec, sig) => {
                self.algin(4);
                self.array(vec, sig, is_le)
            }
            Value::Struct(vec) => {
                self.algin(8);
                self.encode_struct(vec, is_le)
            }
            Value::DictEntry(b) => {
                self.algin(8);
                self.dict_entry(b, is_le)
            }
            Value::Variant(v) => self.variant(v, is_le),
            #[cfg(target_family = "unix")]
            Value::UnixFD(fd) => {
                self.algin(4);
                self.unix_fd(*fd, is_le);
                Ok(())
            }
        }
    }
}
