use super::flags::MessageFlags;
use super::header::MessageHeader;
use super::types::MessageType;
use crate::{Header, Value};
use std::collections::BTreeSet;

/// This represents a DBus [message].
///
/// [message]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone)]
pub struct Message {
    pub(super) header: MessageHeader,
    pub(super) body: Vec<Value>,
}

impl Message {
    /// Create a `Message` object as a MethodCall.
    pub fn method_call(destination: &str, path: &str, interface: &str, member: &str) -> Message {
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
            body: Vec::new(),
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
            headers,
        };
        Message {
            header,
            body: Vec::new(),
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
    pub fn get_path(&self) -> Option<&str> {
        self.header.get_path()
    }

    /// It is true if the message contains an `Interface` in the header fields.
    pub fn has_interface(&self) -> bool {
        self.header.has_interface()
    }

    /// Get the `Path`, if there is one in the header field.
    pub fn get_interface(&self) -> Option<&str> {
        self.header.get_interface()
    }

    /// It is true if the message contains an `Member` in the header fields.
    pub fn has_member(&self) -> bool {
        self.header.has_member()
    }

    /// Get the `Member`, if there is one in the header field.
    pub fn get_member(&self) -> Option<&str> {
        self.header.get_member()
    }

    /// It is true if the message contains an `ErrorName` in the header fields.
    pub fn has_error_name(&self) -> bool {
        self.header.has_error_name()
    }

    /// Get the `ErrorName`, if there is one in the header field.
    pub fn get_error_name(&self) -> Option<&str> {
        self.header.get_error_name()
    }

    /// Get the `Sender`, if there is one in the header field.
    pub fn get_sender(&self) -> Option<&str> {
        self.header.get_sender()
    }

    /// Get the `Destination`, if there is one in the header field.
    pub fn get_destination(&self) -> Option<&str> {
        self.header.get_destination()
    }

    /// Get the `Signature`, if there is one in the header field.
    pub fn get_signature(&self) -> String {
        let mut signature = String::new();
        for v in &self.body {
            v.get_signature(&mut signature);
        }
        signature
    }

    /// Add a new value to the body.
    pub fn add_value(&mut self, value: Value) {
        self.body.push(value);
    }

    /// Create a message return from this `Message`.
    /// Only works if this `Message` is a MethodCall.
    pub fn method_return(&self) -> Result<Message, Message> {
        self.header.method_return()
    }

    /// Create a unknown property error message from this `Message`.
    pub fn unknown_property(&self, property: &str) -> Message {
        self.header.unknown_property(property)
    }

    /// Create a unknown path error message from this `Message`.
    pub fn unknown_path(&self) -> Option<Message> {
        self.header.unknown_path()
    }

    /// Create a unknown interface error message from this `Message`.
    pub fn unknown_interface(&self) -> Option<Message> {
        self.header.unknown_interface()
    }

    /// Create a unknown member error message from this `Message`.
    pub fn unknown_member(&self) -> Option<Message> {
        self.header.unknown_member()
    }

    /// Create an invalid args error message from this `Message`.
    pub fn invalid_args(&self, reason: &str) -> Message {
        self.error(
            "org.freedesktop.DBus.Error.InvalidArgs".to_string(),
            reason.to_string(),
        )
    }

    /// Create an error message from this `Message`.
    pub fn error(&self, name: String, message: String) -> Message {
        self.header.error(name, message)
    }

    /// Get the body.
    pub fn get_body(&self) -> &[Value] {
        &self.body
    }

    /// Get the message type.
    pub fn get_type(&self) -> MessageType {
        self.header.message_type.clone()
    }

    /// Split the `Message` object into the header and the body.
    pub fn split(mut self) -> (MessageHeader, Vec<Value>) {
        let signature = self.get_signature();
        if !signature.is_empty() {
            self.header.headers.insert(Header::Signature(signature));
        }
        (self.header, self.body)
    }
}
