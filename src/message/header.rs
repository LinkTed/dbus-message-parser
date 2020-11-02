use super::message::Message;
use super::{MessageFlags, MessageType};
use crate::{Bus, Error, Header, Interface, Member, ObjectPath, Value};
use std::collections::BTreeSet;
use std::convert::TryInto;

macro_rules! get_header {
    ($self:ident, $enum_case:ident) => {
        for h in &$self.headers {
            if let Header::$enum_case(value) = h {
                return Some(value);
            }
        }

        return None;
    };
}

/// This represents a DBus [message header].
///
/// [message header]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct MessageHeader {
    pub(super) is_le: bool,
    pub(super) message_type: MessageType,
    pub(super) message_flags: MessageFlags,
    pub(super) version: u8,
    pub(super) serial: u32,
    pub(super) headers: BTreeSet<Header>,
}

impl MessageHeader {
    /// Get the `Path`, if there is one in the header field.
    pub fn get_path(&self) -> Option<&ObjectPath> {
        get_header!(self, Path);
    }

    /// Get the `Interface`, if there is one in the header field.
    pub fn get_interface(&self) -> Option<&Interface> {
        get_header!(self, Interface);
    }

    /// It is true if the message contains an `Interface` in the header fields.
    pub fn has_interface(&self) -> bool {
        self.get_interface().is_some()
    }

    ///headers Get the `Member`, if there is one in the header field.
    pub fn get_member(&self) -> Option<&Member> {
        get_header!(self, Member);
    }

    /// It is true if the message contains an `Member` in the header fields.
    pub fn has_member(&self) -> bool {
        self.get_member().is_some()
    }

    /// Get the `ErrorName`, if there is one in the header field.
    pub fn get_error_name(&self) -> Option<&Error> {
        get_header!(self, ErrorName);
    }

    /// It is true if the message contains an `ErrorName` in the header fields.
    pub fn has_error_name(&self) -> bool {
        self.get_error_name().is_some()
    }

    /// Get the `Destination`, if there is one in the header field.
    pub fn get_destination(&self) -> Option<&Bus> {
        get_header!(self, Destination);
    }

    /// Get the `Sender`, if there is one in the header field.
    pub fn get_sender(&self) -> Option<&Bus> {
        get_header!(self, Sender);
    }

    /// Get the serial number.
    pub fn get_serial(&self) -> u32 {
        self.serial
    }

    /// Get the `ReplySerial` number, if there is one in the header field.
    pub fn get_reply_serial(&self) -> Option<u32> {
        for h in &self.headers {
            if let Header::ReplySerial(serial) = h {
                return Some(*serial);
            }
        }

        None
    }

    /// Get the `Signature`, if there is one in the header field.
    pub fn get_signature(&self) -> Option<&str> {
        get_header!(self, Signature);
    }

    /// It is true if the message contains an `Signature` in the header fields.
    pub fn has_signature(&self) -> bool {
        self.get_signature().is_some()
    }

    /// Get the `UnixFDs`, if there is one in the header field.
    pub fn get_unix_fds(&self) -> Option<u32> {
        for h in &self.headers {
            if let Header::UnixFDs(fds) = h {
                return Some(*fds);
            }
        }

        None
    }

    /// It is true if the message contains an `UnixFDs` in the header fields.
    pub fn has_unix_fds(&self) -> bool {
        self.get_unix_fds().is_some()
    }

    /// Create a message return from this `Message`.
    /// Only works if this `Message` is a MethodCall.
    pub fn method_return(&self) -> Result<Message, Message> {
        if let MessageType::MethodCall = self.message_type {
            let message_type = MessageType::MethodReturn;

            let message_flags = MessageFlags::NO_REPLY_EXPECTED;

            let mut headers = BTreeSet::<Header>::new();

            if let Some(sender) = self.get_sender() {
                headers.insert(Header::Destination(sender.clone()));
            }

            if let Some(destination) = self.get_destination() {
                headers.insert(Header::Sender(destination.clone()));
            }

            headers.insert(Header::ReplySerial(self.get_serial()));

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
                body: Vec::new(),
            })
        } else {
            Err(self.error(
                "org.freedesktop.DBus.Error.MessageType".try_into().unwrap(),
                "Message is not a method call".to_string(),
            ))
        }
    }

    /// Create a unknown property error message from this `Message`.
    pub fn unknown_property(&self, property: &str) -> Message {
        let message = format!("does not have a property {}", property);
        self.error(
            "org.freedesktop.DBus.Error.UnknownProperty"
                .try_into()
                .unwrap(),
            message,
        )
    }

    /// Create a unknown path error message from this `Message`.
    pub fn unknown_path(&self) -> Option<Message> {
        if let Some(path) = self.get_path() {
            let message = format!("does not have a path {}", path);
            Some(self.error(
                "org.freedesktop.DBus.Error.UnknownPath".try_into().unwrap(),
                message,
            ))
        } else {
            None
        }
    }

    /// Create a unknown interface error message from this `Message`.
    pub fn unknown_interface(&self) -> Option<Message> {
        if let Some(interface) = self.get_interface() {
            let message = format!("does not have an interface {}", interface);
            Some(
                self.error(
                    "org.freedesktop.DBus.Error.UnknownInterface"
                        .try_into()
                        .unwrap(),
                    message,
                ),
            )
        } else {
            None
        }
    }

    /// Create a unknown member error message from this `Message`.
    pub fn unknown_member(&self) -> Option<Message> {
        if let Some(member) = self.get_member() {
            let message = format!("does not have a member {}", member);
            Some(
                self.error(
                    "org.freedesktop.DBus.Error.UnknownMember"
                        .try_into()
                        .unwrap(),
                    message,
                ),
            )
        } else {
            None
        }
    }

    /// Create an invalid args error message from this `Message`.
    pub fn invalid_args(&self, reason: String) -> Message {
        self.error(
            "org.freedesktop.DBus.Error.InvalidArgs".try_into().unwrap(),
            reason,
        )
    }

    /// Create an error message from this `Message`.
    pub fn error(&self, error: Error, message: String) -> Message {
        let message_type = MessageType::Error;

        let message_flags = MessageFlags::NO_REPLY_EXPECTED;

        let mut headers = BTreeSet::<Header>::new();
        if let Some(sender) = self.get_sender() {
            headers.insert(Header::Destination(sender.clone()));
        }

        if let Some(destination) = self.get_destination() {
            headers.insert(Header::Sender(destination.clone()));
        }
        headers.insert(Header::ReplySerial(self.get_serial()));
        headers.insert(Header::ErrorName(error));

        let header = MessageHeader {
            is_le: self.is_le,
            message_type,
            message_flags,
            version: 1,
            serial: 0,
            headers,
        };
        Message {
            header,
            body: vec![Value::String(message)],
        }
    }

    /// Get the message type.
    pub fn get_type(&self) -> MessageType {
        self.message_type.clone()
    }
}
