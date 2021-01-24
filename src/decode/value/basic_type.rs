use crate::decode::{DecodeError, DecodeResult, Decoder};
use crate::value::{ObjectPath, Type, Value};
use bytes::{Buf, Bytes};
#[cfg(target_family = "unix")]
use std::cmp::max;
use std::convert::TryFrom;
use std::mem::size_of;
use std::str::from_utf8;

impl<'a> Decoder<'a> {
    /// Decode from a byte array at a specific offset to a `u8`.
    pub(crate) fn b(&mut self) -> DecodeResult<u8> {
        let buf = self.read(size_of::<u8>())?;
        Ok(buf[0])
    }

    /// Decode from a byte array at a specific offset to a [`Value::Byte`].
    ///
    /// [`Value::Byte`]: crate::value::Value::Byte
    pub(crate) fn byte(&mut self) -> DecodeResult<Value> {
        let b = self.b()?;
        Ok(Value::Byte(b))
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Boolean`].
    ///
    /// [`Value::Boolean`]: crate::value::Value::Boolean
    pub(crate) fn boolean(&mut self, is_le: bool) -> DecodeResult<Value> {
        let r = self.u_32(is_le)?;
        match r {
            0 => Ok(Value::Boolean(false)),
            1 => Ok(Value::Boolean(true)),
            x => Err(DecodeError::InvalidBoolean(x)),
        }
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Int16`].
    ///
    /// [`Value::Int16`]: crate::value::Value::Int16
    pub(crate) fn int_16(&mut self, is_le: bool) -> DecodeResult<Value> {
        self.algin(2)?;
        let mut buf = self.read(size_of::<i16>())?;
        let i = if is_le {
            buf.get_i16_le()
        } else {
            buf.get_i16()
        };
        Ok(Value::Int16(i))
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Uint16`].
    ///
    /// [`Value::Uint16`]: crate::value::Value::Uint16
    pub(crate) fn uint_16(&mut self, is_le: bool) -> DecodeResult<Value> {
        self.algin(2)?;
        let mut buf = self.read(size_of::<u16>())?;
        let u = if is_le {
            Value::Uint16(buf.get_u16_le())
        } else {
            Value::Uint16(buf.get_u16())
        };
        Ok(u)
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Int32`].
    ///
    /// [`Value::Int32`]: crate::value::Value::Uint32
    pub(crate) fn int_32(&mut self, is_le: bool) -> DecodeResult<Value> {
        self.algin(4)?;
        let mut buf = self.read(size_of::<i32>())?;
        let i = if is_le {
            buf.get_i32_le()
        } else {
            buf.get_i32()
        };
        Ok(Value::Int32(i))
    }

    /// Check alignment and decode from a byte array at a specific offset to a `u32`.
    pub(crate) fn u_32(&mut self, is_le: bool) -> DecodeResult<u32> {
        self.algin(4)?;
        let mut buf = self.read(size_of::<u32>())?;
        let u = if is_le {
            buf.get_u32_le()
        } else {
            buf.get_u32()
        };
        Ok(u)
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Uint32`].
    ///
    /// [`Value::Uint32`]: crate::value::Value::Uint32
    pub(crate) fn uint_32(&mut self, is_le: bool) -> DecodeResult<Value> {
        let u = self.u_32(is_le)?;
        Ok(Value::Uint32(u))
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::UnixFD`].
    ///
    /// [`Value::UnixFD`]: crate::value::Value::UnixFD
    #[cfg(target_family = "unix")]
    pub(crate) fn unix_fd(&mut self, is_le: bool) -> DecodeResult<Value> {
        let i = self.u_32(is_le)? as usize;
        if let Some(fd) = self.fds.get(i) {
            self.offset_fds = max(Some(i), self.offset_fds);
            Ok(Value::UnixFD(*fd))
        } else {
            Err(DecodeError::NotEnoughFds(self.fds.len(), i))
        }
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Int64`].
    ///
    /// [`Value::Int64`]: crate::value::Value::Int64
    pub(crate) fn int_64(&mut self, is_le: bool) -> DecodeResult<Value> {
        self.algin(8)?;
        let mut buf = self.read(size_of::<i64>())?;
        let i = if is_le {
            buf.get_i64_le()
        } else {
            buf.get_i64()
        };
        Ok(Value::Int64(i))
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Uint64`].
    ///
    /// [`Value::Uint64`]: crate::value::Value::Uint64
    pub(crate) fn uint_64(&mut self, is_le: bool) -> DecodeResult<Value> {
        self.algin(8)?;
        let mut buf = self.read(size_of::<u64>())?;
        let u = if is_le {
            Value::Uint64(buf.get_u64_le())
        } else {
            Value::Uint64(buf.get_u64())
        };
        Ok(u)
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::Double`].
    ///
    /// [`Value::Double`]: crate::value::Value::Double
    pub(crate) fn double(&mut self, is_le: bool) -> DecodeResult<Value> {
        self.algin(8)?;
        let mut buf = self.read(size_of::<f64>())?;
        let f = if is_le {
            buf.get_f64_le()
        } else {
            buf.get_f64()
        };
        Ok(Value::Double(f))
    }

    /// Returns n-`length` [`Bytes`] object and check if the next byte is null, because all string have
    /// to be null terminated.
    ///
    /// [`Bytes`]: bytes::Bytes
    #[inline]
    fn d_string_bytes(&mut self, length: usize) -> DecodeResult<Bytes> {
        let bytes = self.read(length)?;
        match self.b()? {
            0 => Ok(bytes),
            b => Err(DecodeError::StringNotNull(b)),
        }
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`String`].
    /// The size of the length is 4.
    ///
    /// [`String`]: std::string:String
    fn d_u32_string_bytes(&mut self, is_le: bool) -> DecodeResult<Bytes> {
        let string_length = self.u_32(is_le)? as usize;
        self.d_string_bytes(string_length)
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::String`].
    ///
    /// [`Value::String`]: crate::value::Value::String
    pub(crate) fn string(&mut self, is_le: bool) -> DecodeResult<Value> {
        let bytes = self.d_u32_string_bytes(is_le)?;
        let string = from_utf8(bytes.as_ref())?;
        Ok(Value::String(string.to_owned()))
    }

    /// Check alignment and decode from a byte array at a specific offset to a
    /// [`Value::ObjectPath`].
    ///
    /// [`Value::ObjectPath`]: crate::value::Value::ObjectPath
    pub(crate) fn object_path(&mut self, is_le: bool) -> DecodeResult<Value> {
        let bytes = self.d_u32_string_bytes(is_le)?;
        let object_path = ObjectPath::try_from(bytes.as_ref())?;
        Ok(Value::ObjectPath(object_path))
    }

    /// Decode from a byte array at a specific offset to a [`Signature`].
    /// The size of the length is 1.
    ///
    /// [`Signature`]: crate::value::Signature
    pub(crate) fn d_signature(&mut self) -> DecodeResult<Vec<Type>> {
        let signature_length = self.b()? as usize;
        let bytes = self.d_string_bytes(signature_length)?;
        let signature = Type::from_bytes_to_signature(bytes.as_ref())?;
        Ok(signature)
    }

    /// Decode from a byte array at a specific offset to a [`Value::Signature`].
    ///
    /// [`Value::Signature`]: crate::value::Value::Signature
    pub(crate) fn signature(&mut self) -> DecodeResult<Value> {
        let signature = self.d_signature()?;
        Ok(Value::Signature(signature))
    }
}
