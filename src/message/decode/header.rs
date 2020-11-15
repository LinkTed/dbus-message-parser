use crate::{
    DecodeError, DecodeResult, Decoder, Header, MessageFlags, MessageHeader, MessageType, Value,
};
use num_traits::FromPrimitive;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::ops::Deref;

impl<'a, T> Decoder<'a, T>
where
    T: Deref<Target = [u8]>,
{
    pub fn message_header(&mut self) -> DecodeResult<(MessageHeader, Option<(u32, String)>)> {
        let signature = "yyyuua(yv)";

        let is_le = match self.b()? {
            0x6c => true,
            0x42 => false,
            _ => return Err(DecodeError::Endianness),
        };

        // Decode the whole header, with the header fields and without the
        // endianness flag.
        let mut header = self.value(is_le, signature)?;
        if header.len() != 6 {
            return Err(DecodeError::Header);
        }

        // Get the message type.
        let message_type = if let Value::Byte(b) = header.remove(0) {
            if let Some(message_type) = MessageType::from_u8(b) {
                message_type
            } else {
                return Err(DecodeError::Header);
            }
        } else {
            return Err(DecodeError::Header);
        };

        // Get the message flags.
        let message_flags = if let Value::Byte(b) = header.remove(0) {
            if let Some(message_flags) = MessageFlags::from_bits(b) {
                message_flags
            } else {
                return Err(DecodeError::Header);
            }
        } else {
            return Err(DecodeError::Header);
        };

        // Get the major protocol version.
        let version = if let Value::Byte(b) = header.remove(0) {
            b
        } else {
            return Err(DecodeError::Header);
        };

        // Get the length in bytes of the message body.
        let body_length = if let Value::Uint32(u) = header.remove(0) {
            u
        } else {
            return Err(DecodeError::Header);
        };

        // Get the serial number of this message.
        let serial = if let Value::Uint32(u) = header.remove(0) {
            u
        } else {
            return Err(DecodeError::Header);
        };

        let mut body_signature = None;

        // Parse the header fields.
        let mut headers = BTreeSet::new();
        if let Value::Array(a, sig) = header.remove(0) {
            if sig != "(yv)" {
                return Err(DecodeError::Header);
            }
            for h in a {
                let header = Header::try_from(h)?;
                if let Header::Signature(signature) = header {
                    body_signature = Some(signature);
                } else {
                    headers.insert(header);
                }
            }
        } else {
            return Err(DecodeError::Header);
        }

        let message_header =
            MessageHeader::new(is_le, message_type, message_flags, version, serial, headers)?;

        if body_length == 0 {
            if body_signature.is_none() {
                Ok((message_header, None))
            } else {
                Err(DecodeError::BodySignatureMissing)
            }
        } else if let Some(signature) = body_signature {
            let body = Some((body_length, signature));
            Ok((message_header, body))
        } else {
            Err(DecodeError::BodySignatureMissing)
        }
    }
}
