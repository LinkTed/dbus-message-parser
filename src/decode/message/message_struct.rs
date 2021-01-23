use crate::decode::{DecodeError, DecodeResult, Decoder};
use crate::message::Message;
use crate::value::{Type, Value};
use bytes::Bytes;
#[cfg(target_family = "unix")]
use std::os::unix::io::RawFd;

impl<'a> Decoder<'a> {
    fn message_body(
        &mut self,
        is_le: bool,
        length: u32,
        signature: &[Type],
    ) -> DecodeResult<Vec<Value>> {
        let end = Decoder::<'a>::checked_add(self.offset, length as usize)?;
        let body = self.value(is_le, 0, signature)?;
        if end == self.offset {
            Ok(body)
        } else {
            Err(DecodeError::BodyLength(end, self.offset))
        }
    }

    /// Decode a byte array to a `Message` object.
    ///
    /// # Example
    /// ```
    /// # use bytes::Bytes;
    /// # use dbus_message_parser::message::Message;
    /// let msg = b"l\x02\x01\x01\n\0\0\0\x01\0\0\0=\0\0\0\x06\x01s\0\x05\0\0\0:1.98\0\0\0\x05\x01u\0\x01\0\0\0\x08\x01g\0\x01s\0\0\x07\x01s\0\x14\0\0\0org.freedesktop.DBus\0\0\0\0\x05\0\0\0:1.98\0";
    /// let buf_read = Bytes::copy_from_slice(&msg[..]);
    ///
    /// if let Ok(msg) = Message::decode(buf_read) {
    ///     //...
    /// }
    /// ```
    pub(crate) fn message(&mut self) -> DecodeResult<Message> {
        let (header, body) = self.message_header()?;

        self.algin(8)?;

        let body = match body {
            Some((body_length, body_signature)) => {
                self.message_body(header.is_le, body_length, &body_signature)?
            }
            None => Vec::new(),
        };

        Ok(Message::new(header, body))
    }
}

impl Message {
    /// The decode a [`Message`] and returns the offset.
    pub fn decode(bytes: Bytes) -> DecodeResult<(Message, usize)> {
        let mut decoder = Decoder::new(bytes);
        let msg = decoder.message()?;
        let offset = decoder.offset;
        Ok((msg, offset))
    }

    /// The decode a [`Message`] and returns the offset and the offset of the given FDs.
    #[cfg(target_family = "unix")]
    pub fn decode_with_fds(bytes: Bytes, fds: &[RawFd]) -> DecodeResult<(Message, usize, usize)> {
        let mut decoder = Decoder::new_with_fds(bytes, fds);
        let msg = decoder.message()?;
        let offset = decoder.offset;
        let offset_fds = match decoder.offset_fds {
            Some(offset_fds) => offset_fds + 1,
            None => 0,
        };
        Ok((msg, offset, offset_fds))
    }
}

#[test]
fn message_body_error() {
    let b = Bytes::from_static(b"\xff");
    let type_ = Type::Byte;
    let mut decoder = Decoder::new(b);
    assert_eq!(
        decoder.message_body(true, 2, &[type_]),
        Err(DecodeError::BodyLength(2, 1))
    );
}
