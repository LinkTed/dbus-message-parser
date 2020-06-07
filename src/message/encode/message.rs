use crate::{EncodeError, EncodeResult, Encoder, Message};
use bytes::BytesMut;

impl<'a> Encoder<'a> {
    /// Encode a `Message` object to a byte array.
    pub fn message(&mut self, message: &Message) -> EncodeResult {
        let is_le = message.header.is_le;
        // Encode the body in another buffer
        let mut buf_body = BytesMut::with_capacity(1024);
        let mut encoder = Encoder::new(&mut buf_body, self.fds);
        let mut body_signature = String::new();
        for v in &message.body {
            v.get_signature(&mut body_signature);
            encoder.value(v, is_le)?;
        }

        let body_length = buf_body.len() as u32;
        if body_length == 0 {
            if body_signature.is_empty() {
                self.message_header(&message.header, None)?;
            } else {
                return Err(EncodeError::NullSignature);
            }
        } else if body_signature.is_empty() {
            return Err(EncodeError::NullSignature);
        } else {
            let body = Some((body_length, body_signature));
            self.message_header(&message.header, body)?;
        }

        // Append the body.
        self.algin(8);
        self.buf.extend(buf_body);

        Ok(())
    }
}
