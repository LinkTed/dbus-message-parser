use bytes::{BufMut, BytesMut};
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

pub struct Encoder {
    pub(crate) buf: BytesMut,
    #[cfg(target_family = "unix")]
    pub(crate) fds: Vec<RawFd>,
}

impl Encoder {
    /// This is a helper function to add the algin to the buffer.
    pub(crate) fn algin(&mut self, a: usize) {
        let remain = self.buf.len() % a;
        if remain != 0 {
            let padding_length = a - remain;
            self.buf.reserve(padding_length);

            for _ in 0..padding_length {
                self.buf.put_u8(0);
            }
        }
    }

    pub fn new() -> Encoder {
        Encoder {
            buf: BytesMut::new(),
            #[cfg(target_family = "unix")]
            fds: Vec::new(),
        }
    }
}
