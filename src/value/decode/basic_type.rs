use crate::{DecodeError, DecodeResult, Decoder, Value, OBJECT_PATH_REGEX};
use bytes::Buf;
#[cfg(target_family = "unix")]
use std::cmp::max;
use std::mem::size_of;
use std::ops::Deref;

impl<'a, T> Decoder<'a, T>
where
    T: Deref<Target = [u8]>,
{
    pub(crate) fn b(&mut self) -> DecodeResult<u8> {
        let start = self.offset;
        self.offset += size_of::<u8>();

        if let Some(buf) = self.buf.get(start..self.offset) {
            Ok(buf[0])
        } else {
            Err(DecodeError::TooShort)
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::Byte`.
    pub fn byte(&mut self) -> DecodeResult<Value> {
        let b = self.b()?;
        Ok(Value::Byte(b))
    }

    /// Decode from a byte array at a specific offset to a `Value::Boolean`.
    pub fn boolean(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<u32>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                buf.get_u32_le()
            } else {
                buf.get_u32()
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        match r {
            0 => Ok(Value::Boolean(false)),
            1 => Ok(Value::Boolean(true)),
            x => Err(DecodeError::InvalidBoolean(x)),
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::Int16`.
    pub fn int_16(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<i16>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                Value::Int16(buf.get_i16_le())
            } else {
                Value::Int16(buf.get_i16())
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    /// Decode from a byte array at a specific offset to a `Value::Uint16`.
    pub fn uint_16(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<u16>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                Value::Uint16(buf.get_u16_le())
            } else {
                Value::Uint16(buf.get_u16())
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    /// Decode from a byte array at a specific offset to a `Value::Int32`.
    pub fn int_32(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<i32>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                Value::Int32(buf.get_i32_le())
            } else {
                Value::Int32(buf.get_i32())
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    pub(crate) fn u_32(&mut self, is_le: bool) -> DecodeResult<u32> {
        let start = self.offset;
        self.offset += size_of::<u32>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                buf.get_u32_le()
            } else {
                buf.get_u32()
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    /// Decode from a byte array at a specific offset to a `Value::Uint32`.
    pub fn uint_32(&mut self, is_le: bool) -> DecodeResult<Value> {
        let u = self.u_32(is_le)?;
        Ok(Value::Uint32(u))
    }

    /// Decode from a byte array at a specific offset to a `Value::UnixFD`.
    #[cfg(target_family = "unix")]
    pub fn unix_fd(&mut self, is_le: bool) -> DecodeResult<Value> {
        let i = self.u_32(is_le)? as usize;
        if let Some(fd) = self.fds.get(i) {
            self.offset_fds = max(i, self.offset_fds);
            Ok(Value::UnixFD(*fd))
        } else {
            Err(DecodeError::OutOfBoundsFds)
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::Int64`.
    pub fn int_64(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<i64>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                Value::Int64(buf.get_i64_le())
            } else {
                Value::Int64(buf.get_i64())
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    /// Decode from a byte array at a specific offset to a `Value::Uint64`.
    pub fn uint_64(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<u64>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                Value::Uint64(buf.get_u64_le())
            } else {
                Value::Uint64(buf.get_u64())
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    /// Decode from a byte array at a specific offset to a `Value::Double`.
    pub fn double(&mut self, is_le: bool) -> DecodeResult<Value> {
        let start = self.offset;
        self.offset += size_of::<f64>();

        let r = if let Some(mut buf) = self.buf.get(start..self.offset) {
            if is_le {
                Value::Double(buf.get_f64_le())
            } else {
                Value::Double(buf.get_f64())
            }
        } else {
            return Err(DecodeError::TooShort);
        };

        Ok(r)
    }

    /// Decode from a byte array at a specific offset to a `String`.
    /// The size of the length is 4.
    fn str(&mut self, is_le: bool) -> DecodeResult<String> {
        let string_length = self.u_32(is_le)? as usize;

        let start = self.offset;
        self.offset += string_length + 1;

        if let Some(buf) = self.buf.get(start..self.offset) {
            let string = String::from_utf8(buf[..string_length].to_vec())?;
            if *buf.last().unwrap() == 0 {
                Ok(string)
            } else {
                Err(DecodeError::StringNotNull)
            }
        } else {
            Err(DecodeError::TooShort)
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::String`.
    pub fn string(&mut self, is_le: bool) -> DecodeResult<Value> {
        let s = self.str(is_le)?;
        Ok(Value::String(s))
    }

    /// Decode from a byte array at a specific offset to a `Value::ObjectPath`.
    pub fn path(&mut self, is_le: bool) -> DecodeResult<Value> {
        let s = self.str(is_le)?;

        if OBJECT_PATH_REGEX.is_match(&s) {
            Ok(Value::ObjectPath(s))
        } else {
            Err(DecodeError::ObjectPathRegex)
        }
    }

    /// Decode from a byte array at a specific offset to a `String`.
    /// The size of the length is 1.
    pub(crate) fn sig(&mut self) -> DecodeResult<String> {
        let string_size = self.b()? as usize;

        let start = self.offset;
        self.offset += string_size + 1;

        if let Some(buf) = self.buf.get(start..self.offset) {
            let string = String::from_utf8(buf[..string_size].to_vec())?;
            if *buf.last().unwrap() == 0 {
                Ok(string)
            } else {
                Err(DecodeError::StringNotNull)
            }
        } else {
            Err(DecodeError::TooShort)
        }
    }

    /// Decode from a byte array at a specific offset to a `Value::Signature`.
    pub fn signature(&mut self) -> DecodeResult<Value> {
        let s = self.sig()?;
        Ok(Value::Signature(s))
    }
}
