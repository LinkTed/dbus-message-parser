use crate::{
    decode::{DecodeError, DecodeResult, Decoder},
    message::{MessageFlags, MessageHeader, MessageHeaderFields, MessageType},
    value::Type,
};
#[cfg(test)]
use bytes::Bytes;
use std::convert::TryFrom;

impl<'a> Decoder<'a> {
    fn message_header_type(&mut self) -> DecodeResult<MessageType> {
        let b = self.u_8()?;
        match MessageType::try_from(b) {
            Ok(message_type) => Ok(message_type),
            Err(message_type) => Err(DecodeError::MessageType(message_type)),
        }
    }

    fn message_header_flags(&mut self) -> DecodeResult<MessageFlags> {
        let b = self.u_8()?;
        match MessageFlags::from_bits(b) {
            Some(message_flags) => Ok(message_flags),
            None => Err(DecodeError::MessageFlags(b)),
        }
    }

    fn message_header_fields(
        &mut self,
        is_le: bool,
    ) -> DecodeResult<(Option<Vec<Type>>, MessageHeaderFields)> {
        let signature = Type::Struct(vec![Type::Byte, Type::Variant]);
        let array = self.d_array(is_le, 0, &signature)?;
        let mut fields = MessageHeaderFields::try_from(array)?;
        let body_signature = fields.signature.take();
        Ok((body_signature, fields))
    }

    fn message_header_is_le(&mut self) -> DecodeResult<bool> {
        match self.u_8()? {
            0x6c => Ok(true),
            0x42 => Ok(false),
            b => Err(DecodeError::Endianness(b)),
        }
    }

    pub fn message_header(&mut self) -> DecodeResult<(MessageHeader, Option<(u32, Vec<Type>)>)> {
        let is_le = self.message_header_is_le()?;

        // Get the message type.
        let message_type = self.message_header_type()?;

        // Get the message flags.
        let message_flags = self.message_header_flags()?;

        // Get the major protocol version.
        let version = self.u_8()?;

        // Get the length in bytes of the message body.
        let body_length = self.u_32(is_le)?;

        // Get the serial number of this message.
        let serial = self.u_32(is_le)?;

        // Parse the header fields.
        let (body_signature, headers) = self.message_header_fields(is_le)?;

        let message_header =
            MessageHeader::new(is_le, message_type, message_flags, version, serial, headers)?;

        if body_length == 0 {
            match body_signature {
                Some(signature) => Err(DecodeError::BodyLengthZero(signature)),
                None => Ok((message_header, None)),
            }
        } else {
            match body_signature {
                Some(signature) => Ok((message_header, Some((body_length, signature)))),
                None => Err(DecodeError::BodySignatureMissing(body_length)),
            }
        }
    }
}

#[test]
fn message_header_type_error() {
    let b = Bytes::from_static(b"\x11");
    let mut decoder = Decoder::new(b);
    assert_eq!(
        decoder.message_header_type(),
        Err(DecodeError::MessageType(0x11))
    );
}

#[test]
fn message_header_flags_error() {
    let b = Bytes::from_static(b"\xff");
    let mut decoder = Decoder::new(b);
    assert_eq!(
        decoder.message_header_flags(),
        Err(DecodeError::MessageFlags(0xff))
    );
}

#[test]
fn message_header_is_le_big_endian() {
    let b = Bytes::from_static(b"B");
    let mut decoder = Decoder::new(b);
    assert_eq!(decoder.message_header_is_le(), Ok(false));
}

#[test]
fn message_header_is_le_error() {
    let b = Bytes::from_static(b"\xff");
    let mut decoder = Decoder::new(b);
    assert_eq!(
        decoder.message_header_is_le(),
        Err(DecodeError::Endianness(0xff))
    );
}

#[test]
fn message_body_signature_error() {
    let b = Bytes::from_static(
        b"\x6c\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x64\x00\x00\x00\
    \x01\x01\x6f\x00\x0c\x00\x00\x00\x2f\x6f\x62\x6a\x65\x63\x74\x2f\x70\x61\x74\x68\x00\x00\x00\
    \x00\x02\x01\x73\x00\x0e\x00\x00\x00\x69\x6e\x74\x65\x72\x66\x61\x63\x65\x2e\x6e\x61\x6d\x65\
    \x00\x00\x03\x01\x73\x00\x0a\x00\x00\x00\x4d\x65\x74\x68\x6f\x64\x4e\x61\x6d\x65\x00\x00\x00\
    \x00\x00\x00\x06\x01\x73\x00\x13\x00\x00\x00\x64\x65\x73\x74\x69\x6e\x61\x74\x69\x6f\x6e\x2e\
    \x61\x64\x64\x72\x65\x73\x73\x00\x00\x00\x00\x00",
    );
    let mut decoder = Decoder::new(b);
    assert_eq!(
        decoder.message_header(),
        Err(DecodeError::BodySignatureMissing(1))
    );
}
