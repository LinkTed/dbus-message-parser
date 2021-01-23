use crate::encode::{EncodeResult, Encoder};
use crate::message::{MessageHeader, MessageHeaderField};
use crate::value::{Array, Type, Value};
use cfg_if::cfg_if;
use lazy_static::lazy_static;
use std::convert::TryInto;

lazy_static! {
    static ref ARRAY_TYPE: Type = "(yv)".try_into().unwrap();
}

impl Encoder {
    pub fn message_header(
        &mut self,
        message_header: &MessageHeader,
        body: Option<(u32, Vec<Type>)>,
    ) -> EncodeResult<()> {
        let is_le = message_header.is_le;

        // Endianness flag
        if is_le {
            self.byte(0x6c)
        } else {
            self.byte(0x42)
        }
        // Message type
        self.byte(message_header.message_type.clone() as u8);
        // Message flags
        self.byte(message_header.message_flags.bits());
        // Major protocol version
        self.byte(message_header.version);

        // Add the signature of the body to the header fields
        let mut fields = message_header.fields.clone();
        cfg_if! {
            if #[cfg(target_family = "unix")] {
                let fds_len = self.fds.len();
                if fds_len != 0 {
                    fields.insert(MessageHeaderField::UnixFDs(fds_len as u32));
                }
            }
        }
        let body_length = if let Some((body_length, body_signature)) = body {
            fields.insert(MessageHeaderField::Signature(body_signature));
            body_length
        } else {
            0
        };

        self.u_32(body_length, is_le);
        self.u_32(message_header.serial, is_le);

        // Encode the header fields.
        let headers: Vec<Value> = fields.into_iter().map(Value::from).collect();
        let headers = Array {
            array: headers,
            type_: ARRAY_TYPE.clone(),
        };
        self.array(&headers, is_le)?;

        Ok(())
    }
}
