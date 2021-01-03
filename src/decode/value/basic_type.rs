use crate::decode::{DecodeError, DecodeResult, Decoder};
use crate::value::{ObjectPath, Signature, Value};
use bytes::Buf;
#[cfg(target_family = "unix")]
use std::cmp::max;
use std::convert::{TryFrom, TryInto};
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

    /// Computes `self.offset += rhs + 1` and check if there null-terminated string of the length
    /// of `rhs + 1`.
    ///
    /// Return a [`DecodeError::OffsetOverflow`] if a overflow occours, else the `String` wihout
    /// the null.
    ///
    /// [`DecodeError::OffsetOverflow`]: dbus_message_parser::DecodeError
    #[inline]
    fn d_string(&mut self, rhs: usize) -> DecodeResult<String> {
        let length = Decoder::<'a>::checked_add(1, rhs)?;
        let bytes = self.read(length)?;
        let string = from_utf8(bytes.as_ref())?;
        let last = length - 1; // The position of the last char
        match bytes[last] {
            0 => Ok(string[..last].to_owned()),
            b => Err(DecodeError::StringNotNull(b)),
        }
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`String`].
    /// The size of the length is 4.
    ///
    /// [`String`]: std::string:String
    fn d_u32_string(&mut self, is_le: bool) -> DecodeResult<String> {
        let string_length = self.u_32(is_le)? as usize;
        self.d_string(string_length)
    }

    /// Check alignment and decode from a byte array at a specific offset to a [`Value::String`].
    ///
    /// [`Value::String`]: crate::value::Value::String
    pub(crate) fn string(&mut self, is_le: bool) -> DecodeResult<Value> {
        let s = self.d_u32_string(is_le)?;
        Ok(Value::String(s))
    }

    /// Check alignment and decode from a byte array at a specific offset to a
    /// [`Value::ObjectPath`].
    ///
    /// [`Value::ObjectPath`]: crate::value::Value::ObjectPath
    pub(crate) fn object_path(&mut self, is_le: bool) -> DecodeResult<Value> {
        let s = self.d_u32_string(is_le)?;
        let o = ObjectPath::try_from(s)?;
        Ok(Value::ObjectPath(o))
    }

    /// Decode from a byte array at a specific offset to a [`Signature`].
    /// The size of the length is 1.
    ///
    /// [`Signature`]: crate::value::Signature
    pub(crate) fn d_signature(&mut self) -> DecodeResult<Signature> {
        let signature_length = self.b()? as usize;
        let signature = self.d_string(signature_length)?;
        let signature = signature.try_into()?;
        Ok(signature)
    }

    /// Decode from a byte array at a specific offset to a [`Value::Signature`].
    ///
    /// [`Value::Signature`]: crate::value::Value::Signature
    pub(crate) fn signature(&mut self) -> DecodeResult<Value> {
        let s = self.d_signature()?;
        Ok(Value::Signature(s))
    }
}
