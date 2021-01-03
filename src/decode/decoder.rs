use crate::decode::{DecodeError, DecodeResult};
use crate::message::MAXIMUM_MESSAGE_LENGTH;
use bytes::Bytes;
#[cfg(not(target_family = "unix"))]
use std::marker::PhantomData;
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

fn check_length(buf: Bytes) -> Bytes {
    if buf.len() < MAXIMUM_MESSAGE_LENGTH {
        buf
    } else {
        buf.slice(..MAXIMUM_MESSAGE_LENGTH)
    }
}

pub struct Decoder<'a> {
    pub(crate) buf: Bytes,
    pub(crate) offset: usize,
    #[cfg(target_family = "unix")]
    pub(crate) fds: &'a [RawFd],
    #[cfg(target_family = "unix")]
    pub(crate) offset_fds: Option<usize>,
    #[cfg(not(target_family = "unix"))]
    phantom: PhantomData<&'a usize>,
}

impl<'a> Decoder<'a> {
    /// This is a helper function to add the algin to the offset.
    pub(crate) fn algin(&mut self, a: usize) -> DecodeResult<()> {
        let remain = self.offset % a;
        if remain != 0 {
            let padding_length = a - remain;
            let padding = self.read(padding_length)?;
            for b in padding {
                if b != 0 {
                    return Err(DecodeError::Padding(b));
                }
            }
        }
        Ok(())
    }

    pub fn new(buf: Bytes) -> Decoder<'static> {
        let buf = check_length(buf);
        Decoder {
            buf,
            offset: 0,
            #[cfg(target_family = "unix")]
            fds: &[],
            #[cfg(target_family = "unix")]
            offset_fds: None,
            #[cfg(not(target_family = "unix"))]
            phantom: PhantomData,
        }
    }

    #[cfg(target_family = "unix")]
    pub fn new_with_fds(buf: Bytes, fds: &[RawFd]) -> Decoder<'_> {
        let buf = check_length(buf);
        Decoder {
            buf,
            offset: 0,
            fds,
            offset_fds: None,
        }
    }

    #[inline]
    pub(crate) fn checked_add(left: usize, right: usize) -> DecodeResult<usize> {
        if let Some(result) = left.checked_add(right) {
            Ok(result)
        } else {
            Err(DecodeError::IntegerOverflow(left, right))
        }
    }

    #[inline]
    pub(super) fn read(&mut self, length: usize) -> DecodeResult<Bytes> {
        let start = self.offset;
        self.offset = Decoder::checked_add(start, length)?;
        let buf_len = self.buf.len();
        if self.offset <= buf_len {
            Ok(self.buf.slice(start..self.offset))
        } else {
            Err(DecodeError::NotEnoughBytes(buf_len, self.offset))
        }
    }
}

#[test]
fn check_maximum_length() {
    use bytes::BytesMut;
    let b: [u8; 131072] = [0; 131072]; // 16 KiB
    let mut bytes = BytesMut::new();
    while bytes.len() < MAXIMUM_MESSAGE_LENGTH {
        bytes.extend_from_slice(&b[..]);
    }
    bytes.extend_from_slice(&b[..]);
    let bytes = bytes.freeze();
    let bytes = check_length(bytes);
    assert_eq!(bytes.len(), MAXIMUM_MESSAGE_LENGTH);
}
