use super::{HeaderError, HeaderFields};
use crate::{
    message::{Message, MessageFlags, MessageType},
    value::{Bus, Error, Interface, Member, ObjectPath, Type, Value},
};
use std::convert::TryInto;

/// This represents a DBus [message header].
///
/// [message header]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Header {
    pub(crate) is_le: bool,
    pub(crate) message_type: MessageType,
    pub(crate) message_flags: MessageFlags,
    pub(crate) version: u8,
    pub(crate) serial: u32,
    pub(crate) fields: HeaderFields,
}

impl Header {
    /// Create a [`MessageHeader`] object. It can fail if the required [header fields] are not
    /// present.
    ///
    /// [header fields]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields
    pub fn new(
        is_le: bool,
        message_type: MessageType,
        message_flags: MessageFlags,
        version: u8,
        serial: u32,
        fields: HeaderFields,
    ) -> Result<Header, HeaderError> {
        match message_type {
            MessageType::MethodCall => {
                if fields.path.is_none() {
                    return Err(HeaderError::MissingPath);
                }

                if fields.member.is_none() {
                    return Err(HeaderError::MissingMember);
                }
            }
            MessageType::Signal => {
                if fields.path.is_none() {
                    return Err(HeaderError::MissingPath);
                }

                if fields.interface.is_none() {
                    return Err(HeaderError::MissingInterface);
                }

                if fields.member.is_none() {
                    return Err(HeaderError::MissingMember);
                }
            }
            MessageType::Error => {
                if fields.error_name.is_none() {
                    return Err(HeaderError::MissingErrorName);
                }

                if fields.reply_serial.is_none() {
                    return Err(HeaderError::MissingReplySerial);
                }
            }
            MessageType::MethodReturn => {
                if fields.reply_serial.is_none() {
                    return Err(HeaderError::MissingReplySerial);
                }
            }
        }

        let header = Header {
            is_le,
            message_type,
            message_flags,
            version,
            serial,
            fields,
        };

        Ok(header)
    }

    /// Get the serial number.
    #[inline]
    pub const fn get_serial(&self) -> u32 {
        self.serial
    }

    /// Get the [`Path`], if there is one in the header field.
    ///
    /// [`Path`]: crate::message::MessageHeaderField::Path
    pub fn get_path(&self) -> Option<&ObjectPath> {
        self.fields.path.as_ref()
    }

    /// It is true if the message contains an [`Path`] in the header fields.
    ///
    /// [`Path`]: crate::message::MessageHeaderField::Path
    #[inline]
    pub fn has_path(&self) -> bool {
        self.fields.path.is_some()
    }

    /// Get the [`Interface`], if there is one in the header field.
    ///
    /// [`Interface`]: crate::message::MessageHeaderField::Interface
    pub fn get_interface(&self) -> Option<&Interface> {
        self.fields.interface.as_ref()
    }

    /// It is true if the message contains an [`Interface`] in the header fields.
    ///
    /// [`Interface`]: crate::message::MessageHeaderField::Interface
    #[inline]
    pub fn has_interface(&self) -> bool {
        self.fields.interface.is_some()
    }

    /// Get the [`Member`], if there is one in the header field.
    ///
    /// [`Member`]: crate::message::MessageHeaderField::Member
    pub fn get_member(&self) -> Option<&Member> {
        self.fields.member.as_ref()
    }

    /// It is true if the message contains an [`Member`] in the header fields.
    ///
    /// [`Member`]: crate::message::MessageHeaderField::Member
    #[inline]
    pub fn has_member(&self) -> bool {
        self.fields.member.is_some()
    }

    /// Get the [`ErrorName`], if there is one in the header field.
    ///
    /// [`ErrorName`]: crate::message::MessageHeaderField::ErrorName
    pub fn get_error_name(&self) -> Option<&Error> {
        self.fields.error_name.as_ref()
    }

    /// It is true if the message contains an [`ErrorName`] in the header fields.
    ///
    /// [`ErrorName`]: crate::message::MessageHeaderField::ErrorName
    #[inline]
    pub fn has_error_name(&self) -> bool {
        self.fields.error_name.is_some()
    }

    /// Get the [`Destination`], if there is one in the header field.
    ///
    /// [`Destination`]: crate::message::MessageHeaderField::Destination
    pub fn get_destination(&self) -> Option<&Bus> {
        self.fields.destination.as_ref()
    }

    /// It is true if the message contains a [`Destination`] in the header fields.
    ///
    /// [`Destination`]: crate::message::MessageHeaderField::Destination
    #[inline]
    pub fn has_destination(&self) -> bool {
        self.fields.destination.is_some()
    }

    /// Get the [`Sender`], if there is one in the header field.
    ///
    /// [`Sender`]: crate::message::MessageHeaderField::Sender
    pub fn get_sender(&self) -> Option<&Bus> {
        self.fields.sender.as_ref()
    }

    /// It is true if the message contains a [`Sender`] in the header fields.
    ///
    /// [`Sender`]: crate::message::MessageHeaderField::Sender
    #[inline]
    pub fn has_sender(&self) -> bool {
        self.fields.sender.is_some()
    }

    /// Get the [`ReplySerial`] number, if there is one in the header field.
    ///
    /// [`ReplySerial`]: crate::message::MessageHeaderField::ReplySerial
    pub fn get_reply_serial(&self) -> Option<u32> {
        self.fields.reply_serial
    }

    /// It is true if the message contains an [`ReplySerial`] in the header fields.
    ///
    /// [`ReplySerial`]: crate::message::MessageHeaderField::ReplySerial
    #[inline]
    pub fn has_replay_serial(&self) -> bool {
        self.fields.reply_serial.is_some()
    }

    /// Get the [`Signature`], if there is one in the header field.
    ///
    /// [`Signature`]: crate::message::MessageHeaderField::Signature
    pub fn get_signature(&self) -> Option<&[Type]> {
        self.fields.signature.as_deref()
    }

    /// It is true if the message contains a [`Signature`] in the header fields.
    ///
    /// [`Signature`]: crate::message::MessageHeaderField::Signature
    #[inline]
    pub fn has_signature(&self) -> bool {
        self.fields.signature.is_some()
    }

    /// Get the [`UnixFDs`], if there is one in the header field.
    ///
    /// [`UnixFDs`]: crate::message::MessageHeaderField::UnixFDs
    #[cfg(target_family = "unix")]
    pub fn get_unix_fds(&self) -> Option<u32> {
        self.fields.unix_fds
    }

    /// It is true if the message contains an [`UnixFDs`] in the header fields.
    ///
    /// [`UnixFDs`]: crate::message::MessageHeaderField::UnixFDs
    #[cfg(target_family = "unix")]
    #[inline]
    pub fn has_unix_fds(&self) -> bool {
        self.fields.unix_fds.is_some()
    }

    /// Create a message return from this [`Message`].
    /// Only works if this [`Message`] is a [`MethodCall`].
    ///
    /// [`Message`]: crate::message::Message
    /// [`MethodCall`]: crate::message::MessageType::MethodCall
    pub fn method_return(&self) -> Result<Message, Message> {
        if let MessageType::MethodCall = self.message_type {
            let message_type = MessageType::MethodReturn;

            let message_flags = MessageFlags::NO_REPLY_EXPECTED;

            let mut fields = HeaderFields::default();

            if let Some(sender) = self.get_sender() {
                fields.destination = Some(sender.clone());
            }

            if let Some(destination) = self.get_destination() {
                fields.sender = Some(destination.clone());
            }

            fields.reply_serial = Some(self.get_serial());

            let header = Header {
                is_le: self.is_le,
                message_type,
                message_flags,
                version: 1,
                serial: 0,
                fields,
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

    /// Create a unknown property error message from this [`Message`].
    ///
    /// [`Message`]: crate::message::Message
    pub fn unknown_property(&self, property: &str) -> Message {
        let message = format!("does not have a property {}", property);
        self.error(
            "org.freedesktop.DBus.Error.UnknownProperty"
                .try_into()
                .unwrap(),
            message,
        )
    }

    /// Create a unknown path error message from this [`Message`].
    ///
    /// [`Message`]: crate::message::Message
    pub fn unknown_path(&self) -> Option<Message> {
        let path = self.get_path()?;
        let message = format!("does not have a path {}", path);
        let msg = self.error(
            "org.freedesktop.DBus.Error.UnknownPath".try_into().unwrap(),
            message,
        );
        Some(msg)
    }

    /// Create a unknown interface error message from this [`Message`].
    ///
    /// [`Message`]: crate::message::Message
    pub fn unknown_interface(&self) -> Option<Message> {
        let interface = self.get_interface()?;
        let message = format!("does not have an interface {}", interface);
        let msg = self.error(
            "org.freedesktop.DBus.Error.UnknownInterface"
                .try_into()
                .unwrap(),
            message,
        );
        Some(msg)
    }

    /// Create a unknown member error message from this [`Message`].
    ///
    /// [`Message`]: crate::message::Message
    pub fn unknown_member(&self) -> Option<Message> {
        let member = self.get_member()?;
        let message = format!("does not have a member {}", member);
        let msg = self.error(
            "org.freedesktop.DBus.Error.UnknownMember"
                .try_into()
                .unwrap(),
            message,
        );
        Some(msg)
    }

    /// Create an invalid args error message from this [`Message`].
    ///
    /// [`Message`]: crate::message::Message
    pub fn invalid_args(&self, reason: String) -> Message {
        self.error(
            "org.freedesktop.DBus.Error.InvalidArgs".try_into().unwrap(),
            reason,
        )
    }

    /// Create an error message from this [`Message`].
    ///
    /// [`Message`]: crate::message::Message
    pub fn error(&self, error: Error, message: String) -> Message {
        let message_type = MessageType::Error;

        let message_flags = MessageFlags::NO_REPLY_EXPECTED;

        let mut fields = HeaderFields::default();

        if let Some(sender) = self.get_sender() {
            fields.destination = Some(sender.clone());
        }

        if let Some(destination) = self.get_destination() {
            fields.sender = Some(destination.clone());
        }

        fields.reply_serial = Some(self.get_serial());
        fields.error_name = Some(error);

        let header = Header {
            is_le: self.is_le,
            message_type,
            message_flags,
            version: 1,
            serial: 0,
            fields,
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
