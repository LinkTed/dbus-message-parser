use crate::{DecodeError, DecodeResult};
use bytes::Buf;
use std::ops::Deref;
use std::os::unix::io::RawFd;

pub struct Decoder<'a, T>
where
    T: Buf + Deref<Target = [u8]>,
{
    pub(crate) buf: &'a T,
    pub(crate) offset: usize,
    pub(crate) fds: &'a [RawFd],
    pub(crate) offset_fds: usize,
}

impl<'a, T> Decoder<'a, T>
where
    T: Buf + Deref<Target = [u8]>,
{
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

    pub fn new(buf: &'a T) -> Decoder<'a, T> {
        Decoder {
            buf,
            offset: 0,
            fds: &[],
            offset_fds: 0,
        }
    }

    pub fn with_offset(buf: &'a T, offset: usize) -> Decoder<'a, T> {
        Decoder {
            buf,
            offset,
            fds: &[],
            offset_fds: 0,
        }
    }

    pub fn with_fds(
        buf: &'a T,
        offset: usize,
        fds: &'a [RawFd],
        offset_fds: usize,
    ) -> Decoder<'a, T> {
        Decoder {
            buf,
            offset,
            fds,
            offset_fds,
        }
    }
}
