use crate::{DecodeError, DecodeResult, Decoder, Message};
use bytes::Buf;
use std::ops::Deref;

impl<'a, T> Decoder<'a, T>
where
    T: Buf + Deref<Target = [u8]>,
{
    /// Decode a byte array to a `Message` object.
    ///
    /// # Example
    /// ```
    /// # use bytes::Bytes;
    /// # use dbus_message_parser::{Message, Decoder};
    /// let msg = b"l\x02\x01\x01\n\0\0\0\x01\0\0\0=\0\0\0\x06\x01s\0\x05\0\0\0:1.98\0\0\0\x05\x01u\0\x01\0\0\0\x08\x01g\0\x01s\0\0\x07\x01s\0\x14\0\0\0org.freedesktop.DBus\0\0\0\0\x05\0\0\0:1.98\0";
    /// let buf_read = Bytes::copy_from_slice(&msg[..]);
    /// let mut decoder = Decoder::new(&buf_read);
    ///
    /// if let Ok(msg) = decoder.message() {
    ///     //...
    /// }
    /// ```
    pub fn message(&mut self) -> DecodeResult<Message> {
        let (header, body) = self.message_header()?;

        self.algin(8)?;

        if let Some((body_length, body_signature)) = body {
            let end = self.offset + body_length as usize;
            let is_le = header.is_le;
            let body = self.value(is_le, &body_signature)?;
            if end == self.offset {
                Ok(Message { header, body })
            } else {
                Err(DecodeError::BodyLength(end, self.offset))
            }
        } else {
            Ok(Message {
                header,
                body: Vec::new(),
            })
        }
    }
}
