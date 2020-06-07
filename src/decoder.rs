use crate::{DecodeError, DecodeResult};
use bytes::Bytes;
use std::os::unix::io::RawFd;

pub struct Decoder<'a> {
    pub(crate) buf: &'a Bytes,
    pub(crate) offset: usize,
    pub(crate) fds: &'a [RawFd],
    pub(crate) offset_fds: usize,
}

impl<'a> Decoder<'a> {
    /// This is a helper function to add the algin to the offset.
    pub(crate) fn algin(&mut self, a: usize) -> DecodeResult<()> {
        while self.offset % a != 0 {
            if let Some(b) = self.buf.get(self.offset) {
                if *b != 0 {
                    return Err(DecodeError::Padding);
                }
            } else {
                return Err(DecodeError::TooShort);
            }
            self.offset += 1;
        }
        Ok(())
    }

    pub fn new(buf: &'a Bytes) -> Decoder<'a> {
        Decoder {
            buf,
            offset: 0,
            fds: &[],
            offset_fds: 0,
        }
    }

    pub fn with_offset(buf: &'a Bytes, offset: usize) -> Decoder {
        Decoder {
            buf,
            offset,
            fds: &[],
            offset_fds: 0,
        }
    }

    pub fn with_fds(
        buf: &'a Bytes,
        offset: usize,
        fds: &'a [RawFd],
        offset_fds: usize,
    ) -> Decoder<'a> {
        Decoder {
            buf,
            offset,
            fds,
            offset_fds,
        }
    }
}
