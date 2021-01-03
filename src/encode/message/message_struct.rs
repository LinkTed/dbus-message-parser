use crate::encode::{EncodeError, EncodeResult, Encoder};
use crate::message::Message;
use bytes::BytesMut;
use cfg_if::cfg_if;
use std::convert::TryInto;
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

impl Encoder {
    /// Encode a `Message` object to a byte array.
    pub fn message(&mut self, message: &Message) -> EncodeResult<()> {
        let is_le = message.header.is_le;
        // Encode the body in another buffer
        let mut encoder = Encoder::new();
        let mut body_signature = String::new();
        for v in &message.body {
            v.get_signature_as_string(&mut body_signature);
            // TODO sec check
            encoder.value(v, is_le)?;
        }

        cfg_if! {
            if #[cfg(target_family = "unix")] {
                self.fds = encoder.fds;
            }
        }

        let body = encoder.buf;
        let body_length = body.len() as u32;
        if body_length == 0 {
            if body_signature.is_empty() {
                self.message_header(&message.header, None)?;
            } else {
                let body_signature = body_signature.try_into()?;
                return Err(EncodeError::BodyLengthZero(body_signature));
            }
        } else if body_signature.is_empty() {
            return Err(EncodeError::BodySignatureMissing(body_length));
        } else {
            let body_signature = body_signature.try_into()?;
            let body = Some((body_length, body_signature));
            self.message_header(&message.header, body)?;
        }

        // Append the body.
        self.algin(8);
        self.buf.extend(body);

        Ok(())
    }
}

impl Message {
    pub fn encode(&self) -> EncodeResult<BytesMut> {
        let mut encoder = Encoder::new();
        encoder.message(self)?;
        Ok(encoder.buf)
    }

    #[cfg(target_family = "unix")]
    pub fn encode_with_fds(&self) -> EncodeResult<(BytesMut, Vec<RawFd>)> {
        let mut encoder = Encoder::new();
        encoder.message(self)?;
        let result = (encoder.buf, encoder.fds);
        Ok(result)
    }
}
