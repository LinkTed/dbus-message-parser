use crate::encode::{EncodeResult, Encoder};
use crate::value::Value;

impl Encoder {
    /// Encode a `Value` object to a byte array.
    pub fn value(&mut self, value: &Value, is_le: bool) -> EncodeResult<()> {
        match value {
            Value::Byte(b) => {
                self.byte(*b);
                Ok(())
            }
            Value::Boolean(b) => {
                self.boolean(*b, is_le);
                Ok(())
            }
            Value::Int16(i) => {
                self.int_16(*i, is_le);
                Ok(())
            }
            Value::Uint16(u) => {
                self.uint_16(*u, is_le);
                Ok(())
            }
            Value::Int32(i) => {
                self.int_32(*i, is_le);
                Ok(())
            }
            Value::Uint32(u) => {
                self.uint_32(*u, is_le);
                Ok(())
            }
            Value::Int64(i) => {
                self.int_64(*i, is_le);
                Ok(())
            }
            Value::Uint64(u) => {
                self.uint_64(*u, is_le);
                Ok(())
            }
            Value::Double(f) => {
                self.double(*f, is_le);
                Ok(())
            }
            Value::ObjectPath(s) => {
                self.object_path(s, is_le);
                Ok(())
            }
            Value::String(s) => {
                self.string(s, is_le);
                Ok(())
            }
            Value::Signature(s) => self.signature(s),
            Value::Array(array) => self.array(array, is_le),
            Value::Struct(struct_) => self.encode_struct(struct_, is_le),
            Value::DictEntry(b) => self.dict_entry(b, is_le),
            Value::Variant(v) => self.variant(v, is_le),
            #[cfg(target_family = "unix")]
            Value::UnixFD(fd) => {
                self.unix_fd(*fd, is_le);
                Ok(())
            }
        }
    }
}
