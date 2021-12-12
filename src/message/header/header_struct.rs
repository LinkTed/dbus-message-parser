use super::{HeaderError, HeaderFields};
use crate::{
    message::{Message, MessageFlags, MessageType},
    value::{Bus, Error, Interface, Member, ObjectPath, Type, Value},
};
use std::convert::TryInto;

macro_rules! get_field {
    ($(#[$meta:meta])* $function:ident, $field:ident, $return:ty $(,$as_ref:ident)?) => {
        $(#[$meta])*
        #[inline]
        pub const fn $function(&self) -> Option<$return> {
            self.fields.$field$(.$as_ref())?
        }
    };
}

macro_rules! has_field {
    ($(#[$meta:meta])* $function:ident, $field:ident) => {
        $(#[$meta])*
        #[inline]
        pub const fn $function(&self) -> bool {
            self.fields.$field.is_some()
        }
    };
}

#[inline]
fn check_header_fields(
    message_type: MessageType,
    fields: &HeaderFields,
) -> Result<(), HeaderError> {
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
    Ok(())
}

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
        check_header_fields(message_type, &fields)?;

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

    get_field!(
        /// Get the [`path`], if there is one in the header field.
        ///
        /// [`path`]: crate::message::MessageHeaderFields::path
        get_path,
        path,
        &ObjectPath,
        as_ref
    );

    has_field!(
        /// It is true if the message contains a [`path`] in the header fields.
        ///
        /// [`path`]: crate::message::MessageHeaderField::path
        has_path,
        path
    );

    get_field!(
        /// Get the [`interface`], if there is one in the header field.
        ///
        /// [`interface`]: crate::message::MessageHeaderFields::interface
        get_interface,
        interface,
        &Interface,
        as_ref
    );

    has_field!(
        /// It is true if the message contains an [`interface`] in the header fields.
        ///
        /// [`interface`]: crate::message::MessageHeaderFields::interface
        has_interface,
        interface
    );

    get_field!(
        /// Get the [`member`], if there is one in the header field.
        ///
        /// [`member`]: crate::message::MessageHeaderFields::member
        get_member,
        member,
        &Member,
        as_ref
    );

    has_field!(
        /// It is true if the message contains a [`member`] in the header fields.
        ///
        /// [`member`]: crate::message::MessageHeaderFields::member
        has_member,
        member
    );

    get_field!(
        /// Get the [`error_name`], if there is one in the header field.
        ///
        /// [`error_name`]: crate::message::MessageHeaderFields::error_name
        get_error_name,
        error_name,
        &Error,
        as_ref
    );

    has_field!(
        /// It is true if the message contains an [`error_name`] in the header fields.
        ///
        /// [`error_name`]: crate::message::MessageHeaderFields::error_name
        has_error_name,
        error_name
    );

    get_field!(
        /// Get the [`destination`], if there is one in the header field.
        ///
        /// [`destination`]: crate::message::MessageHeaderFields::destination
        get_destination,
        destination,
        &Bus,
        as_ref
    );

    has_field!(
        /// It is true if the message contains a [`destination`] in the header fields.
        ///
        /// [`destination`]: crate::message::MessageHeaderFields::destination
        has_destination,
        destination
    );

    get_field!(
        /// Get the [`sender`], if there is one in the header field.
        ///
        /// [`sender`]: crate::message::MessageHeaderFields::sender
        get_sender,
        sender,
        &Bus,
        as_ref
    );

    has_field!(
        /// It is true if the message contains a [`sender`] in the header fields.
        ///
        /// [`sender`]: crate::message::MessageHeaderFields::sender
        has_sender,
        sender
    );

    get_field!(
        /// Get the [`reply_serial`], if there is one in the header field.
        ///
        /// [`reply_serial`]: crate::message::MessageHeaderFields::reply_serial
        get_reply_serial,
        reply_serial,
        u32
    );

    has_field!(
        /// It is true if the message contains a [`reply_serial`] in the header fields.
        ///
        /// [`reply_serial`]: crate::message::MessageHeaderFields::reply_serial
        has_reply_serial,
        reply_serial
    );

    /// Get the [`signature`], if there is one in the header field.
    ///
    /// [`signature`]: crate::message::MessageHeaderFields::signature
    #[inline]
    pub fn get_signature(&self) -> Option<&[Type]> {
        self.fields.signature.as_deref()
    }

    has_field!(
        /// It is true if the message contains a [`signature`] in the header fields.
        ///
        /// [`signature`]: crate::message::MessageHeaderFields::signature
        has_signature,
        signature
    );

    #[cfg(target_family = "unix")]
    get_field!(
        /// Get the [`unix_fds`], if there is one in the header field.
        ///
        /// [`unix_fds`]: crate::message::MessageHeaderFields::unix_fds
        get_unix_fds,
        unix_fds,
        u32
    );

    #[cfg(target_family = "unix")]
    has_field!(
        /// It is true if the message contains a [`unix_fds`] in the header fields.
        ///
        /// [`unix_fds`]: crate::message::MessageHeaderFields::unix_fds
        has_unix_fds,
        unix_fds
    );

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
        self.message_type
    }
}
