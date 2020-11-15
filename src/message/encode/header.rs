use crate::{EncodeResult, Encoder, MessageHeader, MessageHeaderField, Value};
use num_traits::ToPrimitive;

impl<'a> Encoder<'a> {
    pub fn message_header(
        &mut self,
        message_header: &MessageHeader,
        body: Option<(u32, String)>,
    ) -> EncodeResult {
        let is_le = message_header.is_le;

        // Endianness flag
        if is_le {
            self.byte(0x6c)
        } else {
            self.byte(0x42)
        }
        // Message type
        self.byte(message_header.message_type.to_u8().unwrap());
        // Message flags
        self.byte(message_header.message_flags.bits());
        // Major protocol version
        self.byte(message_header.version);

        // Add the signature of the body to the header fields
        let mut fields = message_header.fields.clone();
        let body_length = if let Some((body_length, body_signature)) = body {
            fields.insert(MessageHeaderField::Signature(body_signature));
            body_length
        } else {
            0
        };

        self.uint_32(body_length, is_le);
        self.uint_32(message_header.serial, is_le);

        // Encode the header fields.
        let headers: Vec<Value> = fields.into_iter().map(Value::from).collect();
        self.array(&headers, "(yv)", is_le)?;

        Ok(())
    }
}
