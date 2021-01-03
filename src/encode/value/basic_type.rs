use crate::encode::{EncodeResult, Encoder};
use crate::value::{ObjectPath, Signature, SignatureError};
use bytes::BufMut;
use std::convert::TryFrom;
use std::mem::size_of;
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

impl Encoder {
    /// Encode a `u8` into the buffer.
    pub fn byte(&mut self, b: u8) {
        self.buf.reserve(size_of::<u8>());
        self.buf.put_u8(b);
    }

    /// Apply the alignment and encode a `bool` into the buffer.
    pub fn boolean(&mut self, b: bool, is_le: bool) {
        self.algin(4);
        self.buf.reserve(size_of::<u32>());
        if is_le {
            self.buf.put_u32_le(b as u32);
        } else {
            self.buf.put_u32(b as u32);
        }
    }

    /// Apply the alignment and encode a `i16` into the buffer.
    pub fn int_16(&mut self, i: i16, is_le: bool) {
        self.algin(2);
        self.buf.reserve(size_of::<i16>());
        if is_le {
            self.buf.put_i16_le(i);
        } else {
            self.buf.put_i16(i);
        }
    }

    /// Apply the alignment and encode a `u16` into the buffer.
    pub fn uint_16(&mut self, u: u16, is_le: bool) {
        self.algin(2);
        self.buf.reserve(size_of::<u16>());
        if is_le {
            self.buf.put_u16_le(u);
        } else {
            self.buf.put_u16(u);
        }
    }

    /// Encode a `i32` into the buffer.
    pub fn i_32(&mut self, i: i32, is_le: bool) {
        self.buf.reserve(size_of::<i32>());
        if is_le {
            self.buf.put_i32_le(i);
        } else {
            self.buf.put_i32(i);
        }
    }

    /// Apply the alignment and encode a `i32` into the buffer and.
    pub fn int_32(&mut self, i: i32, is_le: bool) {
        self.algin(4);
        self.i_32(i, is_le);
    }

    /// Encode a `u32` into the buffer.
    pub fn u_32(&mut self, u: u32, is_le: bool) {
        self.buf.reserve(size_of::<u32>());
        if is_le {
            self.buf.put_u32_le(u);
        } else {
            self.buf.put_u32(u);
        }
    }

    /// Apply the alignment and encode a `u32` into the buffer.
    pub fn uint_32(&mut self, u: u32, is_le: bool) {
        self.algin(4);
        self.u_32(u, is_le);
    }

    pub(crate) fn set_uint_32(&mut self, u: u32, offset: usize, is_le: bool) {
        let bytes = if is_le {
            u.to_le_bytes()
        } else {
            u.to_be_bytes()
        };

        self.buf[offset] = bytes[0];
        self.buf[offset + 1] = bytes[1];
        self.buf[offset + 2] = bytes[2];
        self.buf[offset + 3] = bytes[3];
    }

    /// Apply the alignment and encode a [`RawFd`] into the buffer.
    ///
    /// [`RawFd`]: std::os::unix::io::RawFd
    #[cfg(target_family = "unix")]
    pub fn unix_fd(&mut self, fd: RawFd, is_len: bool) {
        let i = if let Some(i) = self.fds.iter().position(|i| *i == fd) {
            i
        } else {
            self.fds.push(fd);
            self.fds.len() - 1
        };
        self.uint_32(i as u32, is_len);
    }

    /// Apply the alignment and encode a `i64` into the buffer.
    pub fn int_64(&mut self, i: i64, is_le: bool) {
        self.algin(8);
        self.buf.reserve(size_of::<i64>());
        if is_le {
            self.buf.put_i64_le(i);
        } else {
            self.buf.put_i64(i);
        }
    }

    /// Apply the alignment and encode a `u64` into the buffer.
    pub fn uint_64(&mut self, u: u64, is_le: bool) {
        self.algin(8);
        self.buf.reserve(size_of::<u64>());
        if is_le {
            self.buf.put_u64_le(u);
        } else {
            self.buf.put_u64(u);
        }
    }

    /// Apply the alignment and encode a `f64` into the buffer.
    pub fn double(&mut self, f: f64, is_le: bool) {
        self.algin(8);
        self.buf.reserve(size_of::<f64>());
        if is_le {
            self.buf.put_f64_le(f);
        } else {
            self.buf.put_f64(f);
        }
    }

    /// Apply the alignment and encode a `&str` into the buffer and use 4 bytes.
    pub fn string(&mut self, s: &str, is_le: bool) {
        let string_len = s.len();
        self.uint_32(string_len as u32, is_le);
        self.buf.reserve(string_len + 1);
        self.buf.put(s.as_bytes());
        self.buf.put_u8(0);
    }

    /// Apply the alignment and encode an `&ObjectPath` into the buffer and use 4 bytes.
    pub fn object_path(&mut self, o: &ObjectPath, is_le: bool) {
        self.string(o.as_ref(), is_le);
    }

    /// Encode a `&Signature` into the buffer and use 1 bytes.
    pub fn signature(&mut self, signature: &Signature) -> EncodeResult<()> {
        let sig = signature.as_ref();
        let sig_len = sig.len();
        let b = match u8::try_from(sig_len) {
            Ok(b) => b,
            Err(_) => return Err(SignatureError::TooBig(sig_len).into()),
        };

        self.byte(b);
        self.buf.reserve(sig_len + 1);
        self.buf.put(sig.as_bytes());
        self.buf.put_u8(0);

        Ok(())
    }
}
