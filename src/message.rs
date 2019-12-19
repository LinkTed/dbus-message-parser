use std::collections::BTreeSet;
use bytes::{BytesMut, BufMut};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::{Value, DecodeError, EncodeError};
use crate::header::Header;
use crate::value::{decode_algin, encode_algin};


/// An enum representing the [message type].
///
/// [message type]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types
#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MessageType {
    MethodCall = 1,
    MethodReturn = 2,
    Error = 3,
    Signal = 4
}

bitflags! {
    /// A struct representing the [message flags].
    ///
    /// [message flags]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-messages
    pub struct MessageFlags: u8 {
        const NO_REPLY_EXPECTED = 0x01;
        const NO_AUTO_START = 0x02;
        const ALLOW_INTERACTIVE_AUTHORIZATION = 0x04;
    }
}

/// This represents a DBus [message].
///
/// [message]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone)]
pub struct Message {
    header: MessageHeader,
    body: Vec<Value>,
}

impl Message {
    /// Decode a byte array to a `Message` object.
    ///
    /// # Example
    /// ```
    /// # use bytes::BytesMut;
    /// # use dbus_message_parser::Message;
    /// let msg = b"l\x02\x01\x01\n\0\0\0\x01\0\0\0=\0\0\0\x06\x01s\0\x05\0\0\0:1.98\0\0\0\x05\x01u\0\x01\0\0\0\x08\x01g\0\x01s\0\0\x07\x01s\0\x14\0\0\0org.freedesktop.DBus\0\0\0\0\x05\0\0\0:1.98\0";
    /// let buf_read = BytesMut::from(&msg[..]);
    /// let mut offset: usize = 0;
    ///
    /// if let Ok(msg) = Message::decode(&buf_read, &mut offset) {
    ///     //...
    /// }
    /// ```
    pub fn decode(buf: &BytesMut, offset: &mut usize)
        -> Result<Message, DecodeError> {
        // Check if there are enough bytes at least for the header
        if buf.len() < 16 {
            return Err(DecodeError::TooShort)
        }

        // Endianness flag
        let is_le = match buf[*offset] {
            0x6c => true,
            0x42 => false,
            _ => return Err(DecodeError::Endianness)
        };

        *offset += 1;

        let signature = "yyyuua(yv)";

        // Decode the whole header, with the header fields and without the
        // endianness flag.
        let mut header = Value::decode(buf, offset, is_le, 0, 0, signature)?;
        if header.len() != 6 {
            return Err(DecodeError::Header);
        }

        // Get the message type.
        let message_type = if let Value::Byte(b) = header.remove(0) {
            if let Some(message_type) =  MessageType::from_u8(b) {
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

        // Parse the header fields.
        let mut headers = BTreeSet::new();
        if let Value::Array(a, sig) = header.remove(0) {
            if sig != "(yv)" {
                return Err(DecodeError::Header)
            }
            for h in a {
                headers.insert(Header::from(h)?);
            }
        } else {
            return Err(DecodeError::Header)
        }

        decode_algin(buf, offset, 8)?;


        let body = if body_length == 0 {
            Vec::<Value>::new()
        } else {
            // Get the signature of the body.
            let mut signature = None;
            for h in &headers {
                if let Header::Signature(s) = h {
                    signature = Some(s);
                    break;
                }
            }

            if let Some(s) = signature {
                // Decode the body.
                Value::decode(buf, offset, is_le, 0, 0,s)?
            } else {
                return Err(DecodeError::BodySignatureMissing)
            }
        };

        let header = MessageHeader {
            is_le,
            message_type,
            message_flags,
            version,
            serial,
            headers
        };

        Ok(Message{header, body})
    }

    /// Encode a `Message` object to a byte array.
    pub fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        // Reserve enough bytes for the header.
        buf.reserve(16);

        let header = &self.header;

        let is_le = header.is_le;

        // Endianness flag
        if is_le {
            buf.put_u8(0x6c)
        } else {
            buf.put_u8(0x42)
        }
        // Message type
        buf.put_u8(header.message_type.to_u8().unwrap());
        // Message flags
        buf.put_u8(header.message_flags.bits);
        // Major protocol version
        buf.put_u8(header.version);

        // Encode the body in another buffer
        let mut buf_body = BytesMut::with_capacity(1024);
        let mut sig_body = String::new();
        for v in &self.body {
            v.get_signature(&mut sig_body);
            v.encode(&mut buf_body, header.is_le)?;
        }

        // Add the signature of the body to the header fields
        let mut headers = header.headers.clone();
        if !sig_body.is_empty() {
            headers.insert(Header::Signature(sig_body));
        }

        if is_le {
            // Length in bytes of the message body.
            buf.put_u32_le(buf_body.len() as u32);
            // The serial number of this message.
            buf.put_u32_le(header.serial);
        } else {
            // Length in bytes of the message body.
            buf.put_u32(buf_body.len() as u32);
            // The serial number of this message
            buf.put_u32(header.serial);
        }

        // Encode the header fields.
        let headers = headers.into_iter().map(|h| h.into_value()).collect();
        let headers = Value::Array(headers, "(yv)".to_string());
        headers.encode(buf, is_le)?;

        // Append the body.
        encode_algin(buf, 8);
        buf.extend(buf_body);

        Ok(())
    }

    /// Create a `Message` object as a MethodCall.
    pub fn method_call(destination: &str, path: &str, interface: &str,
                       member: &str) -> Message {
        let mut headers = BTreeSet::new();

        headers.insert(Header::Destination(destination.to_string()));
        headers.insert(Header::Path(path.to_string()));
        headers.insert(Header::Interface(interface.to_string()));
        headers.insert(Header::Member(member.to_string()));

        let header = MessageHeader {
            is_le: true,
            message_type: MessageType::MethodCall,
            message_flags: MessageFlags::empty(),
            version: 1,
            serial: 0,
            headers,
        };
        Message {
            header,
            body: Vec::new()
        }
    }

    /// Create a `Message` object as a Signal.
    pub fn signal(path: &str, interface: &str, member: &str) -> Message {
        let mut headers = BTreeSet::new();

        headers.insert(Header::Path(path.to_string()));
        headers.insert(Header::Interface(interface.to_string()));
        headers.insert(Header::Member(member.to_string()));

        let header = MessageHeader {
            is_le: true,
            message_type: MessageType::Signal,
            message_flags: MessageFlags::NO_REPLY_EXPECTED,
            version: 1,
            serial: 0,
            headers
        };
        Message {
            header,
            body: Vec::new()
        }
    }

    /// Get the serial number.
    pub fn get_serial(&self) -> u32 {
        self.header.get_serial()
    }

    /// Set the serial number.
    pub fn set_serial(&mut self, serial: u32) {
        self.header.serial = serial;
    }

    /// Get the `ReplySerial` number, if there is one in the header field.
    pub fn get_reply_serial(&self) -> Option<u32> {
        self.header.get_reply_serial()
    }

    /// Get the `Path`, if there is one in the header field.
    pub fn get_path(&self) -> Option<&String> {
        self.header.get_path()
    }

    /// It is true if the message contains an `Interface` in the header fields.
    pub fn has_interface(&self) -> bool {
        self.header.has_interface()
    }

    /// Get the `Path`, if there is one in the header field.
    pub fn get_interface(&self) -> Option<&String> {
        self.header.get_interface()
    }

    /// It is true if the message contains an `Member` in the header fields.
    pub fn has_member(&self) -> bool {
        self.header.has_member()
    }

    /// Get the `Member`, if there is one in the header field.
    pub fn get_member(&self) -> Option<&String> {
        self.header.get_member()
    }

    /// It is true if the message contains an `ErrorName` in the header fields.
    pub fn has_error_name(&self) -> bool {
        self.header.has_error_name()
    }

    /// Get the `ErrorName`, if there is one in the header field.
    pub fn get_error_name(&self) -> Option<&String> {
        self.header.get_error_name()
    }

    /// Get the `Sender`, if there is one in the header field.
    pub fn get_sender(&self) -> Option<&String> {
        self.header.get_sender()
    }

    /// Get the `Destination`, if there is one in the header field.
    pub fn get_destination(&self) -> Option<&String> {
        self.header.get_destination()
    }

    /// It is true if the message contains an `Signature` in the header fields.
    pub fn has_signature(&self) -> bool {
        self.header.has_signature()
    }

    /// Get the `Signature`, if there is one in the header field.
    pub fn get_signature(&self) -> Option<&String> {
        self.header.get_signature()
    }

    /// Add a new value to the body.
    pub fn add_value(&mut self, value: Value) {
        self.body.push(value);
    }

    /// Create a message return from this `Message`.
    /// Only works if this `Message` is a MethodCall.
    pub fn method_return(&self) -> Result<Message, ()> {
        self.header.method_return()
    }

    /// Create a unknown path error message from this `Message`.
    pub fn unknown_path(&self) -> Result<Message, ()> {
        self.header.unknown_path()
    }

    /// Create a unknown interface error message from this `Message`.
    pub fn unknown_interface(&self) -> Result<Message, ()> {
        self.header.unknown_interface()
    }

    /// Create a unknown method error message from this `Message`.
    pub fn unknown_method(&self) -> Result<Message, ()> {
        self.header.unknown_method()
    }

    /// Create an invalid args error message from this `Message`.
    pub fn invalid_args(&self, reason: &str) -> Result<Message, ()> {
        self.error("org.freedesktop.DBus.Error.InvalidArgs".to_string(),
                   reason.to_string())
    }

    /// Create an error message from this `Message`.
    pub fn error(&self, name: String, message: String) -> Result<Message, ()> {
        self.header.error(name, message)
    }

    /// Get the body.
    pub fn get_body(&self) -> &Vec<Value> {
        &self.body
    }

    /// Get the message type.
    pub fn get_type(&self) -> MessageType {
        self.header.message_type.clone()
    }

    /// Split the `Message` object into the header and the body.
    pub fn split(self) -> (MessageHeader, Vec<Value>) {
        (self.header, self.body)
    }
}

/// This represents a DBus [message header].
///
/// [message header]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub is_le: bool,
    pub message_type: MessageType,
    pub message_flags: MessageFlags,
    pub version: u8,
    pub serial: u32,
    pub headers: BTreeSet<Header>
}

impl MessageHeader {
    /// Get the `Path`, if there is one in the header field.
    pub fn get_path(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::Path(path) = h {
                return Some(path)
            }
        }

        None
    }

    /// Get the `Interface`, if there is one in the header field.
    pub fn get_interface(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::Interface(interface) = h {
                return Some(interface)
            }
        }

        None
    }

    /// It is true if the message contains an `Interface` in the header fields.
    pub fn has_interface(&self) -> bool {
        for h in &self.headers {
            if let Header::Interface(_) = h {
                return true
            }
        }

        false
    }

    /// Get the `Member`, if there is one in the header field.
    pub fn get_member(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::Member(member) = h {
                return Some(member)
            }
        }

        None
    }

    /// It is true if the message contains an `Member` in the header fields.
    pub fn has_member(&self) -> bool {
        for h in &self.headers {
            if let Header::Member(_) = h {
                return true
            }
        }

        false
    }

    /// Get the `ErrorName`, if there is one in the header field.
    pub fn get_error_name(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::ErrorName(error_name) = h {
                return Some(error_name)
            }
        }

        None
    }

    /// It is true if the message contains an `ErrorName` in the header fields.
    pub fn has_error_name(&self) -> bool {
        for h in &self.headers {
            if let Header::ErrorName(_) = h {
                return true
            }
        }

        false
    }

    /// Get the `Destination`, if there is one in the header field.
    pub fn get_destination(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::Destination(destination) = h {
                return Some(destination)
            }
        }

        None
    }

    /// Get the `Sender`, if there is one in the header field.
    pub fn get_sender(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::Sender(sender) = h {
                return Some(sender)
            }
        }

        None
    }

    /// Get the serial number.
    pub fn get_serial(&self) -> u32 {
        self.serial
    }

    /// Get the `ReplySerial` number, if there is one in the header field.
    pub fn get_reply_serial(&self) -> Option<u32> {
        for h in &self.headers {
            if let Header::ReplySerial(serial) = h {
                return Some(*serial)
            }
        }

        None
    }

    /// Get the `Signature`, if there is one in the header field.
    pub fn get_signature(&self) -> Option<&String> {
        for h in &self.headers {
            if let Header::Signature(signature) = h {
                return Some(signature)
            }
        }

        None
    }

    /// It is true if the message contains an `Signature` in the header fields.
    pub fn has_signature(&self) -> bool {
        for h in &self.headers {
            if let Header::Signature(_) = h {
                return true
            }
        }

        false
    }

    /// Create a message return from this `Message`.
    /// Only works if this `Message` is a MethodCall.
    pub fn method_return(&self) -> Result<Message, ()> {
        if let MessageType::MethodCall = self.message_type {

            let message_type = MessageType::MethodReturn;

            let message_flags = MessageFlags::NO_REPLY_EXPECTED;

            let mut headers = BTreeSet::<Header>::new();

            if let Some(sender) = self.get_sender() {
                headers.insert(Header::Destination(sender.clone()));
            } else {
                return Err(())
            }

            if let Some(destination) = self.get_destination() {
                headers.insert(Header::Sender(destination.clone()));
            } else {
                return Err(())
            }

            headers.insert(Header::ReplySerial(self.get_serial()));

            let header = MessageHeader {
                is_le: self.is_le,
                message_type,
                message_flags,
                version: 1,
                serial: 0,
                headers
            };
            Ok(Message  {
                header,
                body: Vec::new()
            })
        } else {
            Err(())
        }
    }

    /// Create a unknown path error message from this `Message`.
    pub fn unknown_path(&self) -> Result<Message, ()> {
        if let Some(path) = self.get_path() {
            let message = format!("does not have a path {}", path);
            self.error("org.freedesktop.DBus.Error.UnknownPath".to_string(),
                       message)
        } else {
            Err(())
        }
    }

    /// Create a unknown interface error message from this `Message`.
    pub fn unknown_interface(&self) -> Result<Message, ()> {
        if let Some(interface) = self.get_interface() {
            let message = format!("does not have an interface {}", interface);
            self.error(
                "org.freedesktop.DBus.Error.UnknownInterface".to_string(),
                message)
        } else {
            Err(())
        }
    }

    /// Create a unknown method error message from this `Message`.
    pub fn unknown_method(&self) -> Result<Message, ()> {
        if let Some(member) = self.get_member() {
            let message = format!("does not have a member {}", member);
            self.error("org.freedesktop.DBus.Error.UnknownMethod".to_string(),
                       message)
        } else {
            Err(())
        }
    }

    /// Create an invalid args error message from this `Message`.
    pub fn invalid_args(&self, reason: &str) -> Result<Message, ()> {
        self.error("org.freedesktop.DBus.Error.InvalidArgs".to_string(),
                   reason.to_string())
    }

    /// Create an error message from this `Message`.
    pub fn error(&self, name: String, message: String) -> Result<Message, ()> {
        let message_type = MessageType::Error;

        let message_flags = MessageFlags::NO_REPLY_EXPECTED;

        let mut headers = BTreeSet::<Header>::new();
        if let Some(sender) = self.get_sender() {
            headers.insert(Header::Destination(sender.clone()));
        } else {
            return Err(())
        }
        if let Some(destination) = self.get_destination() {
            headers.insert(Header::Sender(destination.clone()));
        } else {
            return Err(())
        }
        headers.insert(Header::ReplySerial(self.get_serial()));
        headers.insert(Header::ErrorName(name));

        let header = MessageHeader {
            is_le: self.is_le,
            message_type,
            message_flags,
            version: 1,
            serial: 0,
            headers,
        };
        Ok(Message {
            header,
            body: vec![Value::String(message)]
        })
    }
}
