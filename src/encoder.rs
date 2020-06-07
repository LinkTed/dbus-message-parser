use bytes::{BufMut, BytesMut};
use std::os::unix::io::RawFd;

pub struct Encoder<'a> {
    pub(crate) buf: &'a mut BytesMut,
    pub(crate) fds: &'a mut Vec<RawFd>,
}

impl<'a> Encoder<'a> {
    /// This is a helper function to add the algin to the buffer.
    pub(crate) fn algin(&mut self, a: usize) {
        let p = self.buf.len() % a;
        if p != 0 {
            let mut p = a - p;
            self.buf.reserve(p);

            while p != 0 {
                self.buf.put_u8(0);
                p -= 1;
            }
        }
    }

    pub fn new(buf: &'a mut BytesMut, fds: &'a mut Vec<RawFd>) -> Encoder<'a> {
        Encoder { buf, fds }
    }
}
