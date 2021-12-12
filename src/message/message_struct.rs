use crate::{
    message::{
        header::{Header, HeaderFields},
        MessageFlags, MessageType,
    },
    value::{Bus, Error, Interface, Member, ObjectPath, Type, TypeError, Value},
};
use std::convert::TryInto;

macro_rules! get_field {
    ($(#[$meta:meta])* $function:ident, $return:ty) => {
        $(#[$meta])*
        #[inline]
        pub const fn $function(&self) -> Option<$return> {
            self.header.$function()
        }
    };
}

macro_rules! has_field {
    ($(#[$meta:meta])* $function:ident) => {
        $(#[$meta])*
        #[inline]
        pub const fn $function(&self) -> bool {
            self.header.$function()
        }
    };
}

/// This represents a DBus [message].
///
/// [message]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Message {
    pub(crate) header: Header,
    pub(crate) body: Vec<Value>,
}

impl Message {
    /// Create a [`Message`] object.
    pub fn new(header: Header, body: Vec<Value>) -> Message {
        Message { header, body }
    }

    /// Create a [`Message`] object as a [`MethodCall`].
    ///
    /// [`MethodCall`]: crate::message::MessageType::MethodCall
    pub fn method_call(
        destination: Bus,
        object_path: ObjectPath,
        interface: Interface,
        member: Member,
    ) -> Message {
        let fields = HeaderFields {
            destination: Some(destination),
            path: Some(object_path),
            interface: Some(interface),
            member: Some(member),
            ..Default::default()
        };

        let header = Header {
            is_le: true,
            message_type: MessageType::MethodCall,
            message_flags: MessageFlags::empty(),
            version: 1,
            serial: 0,
            fields,
        };
        Message {
            header,
            body: Vec::new(),
        }
    }

    /// Create a [`Message`] object as a [`Signal`].
    ///
    /// [`Signal`]: crate::message::MessageType::Signal
    pub fn signal(object_path: ObjectPath, interface: Interface, member: Member) -> Message {
        let fields = HeaderFields {
            path: Some(object_path),
            interface: Some(interface),
            member: Some(member),
            ..Default::default()
        };

        let header = Header {
            is_le: true,
            message_type: MessageType::Signal,
            message_flags: MessageFlags::NO_REPLY_EXPECTED,
            version: 1,
            serial: 0,
            fields,
        };
        Message {
            header,
            body: Vec::new(),
        }
    }

    /// Create a [`Message`] to retrieve property value.
    pub fn property_get(
        destination: Bus,
        object_path: ObjectPath,
        interface: Interface,
        property: &str,
    ) -> Message {
        let mut msg = Message::method_call(
            destination,
            object_path,
            "org.freedesktop.DBus.Properties".try_into().unwrap(),
            "Get".try_into().unwrap(),
        );

        msg.add_value(Value::String(interface.to_string()));
        msg.add_value(Value::String(property.to_string()));

        msg
    }

    /// Create a [`Message`] to retrieve property value.
    pub fn properties_get_all(
        destination: Bus,
        object_path: ObjectPath,
        interface: Interface,
    ) -> Message {
        let mut msg = Message::method_call(
            destination,
            object_path,
            "org.freedesktop.DBus.Properties".try_into().unwrap(),
            "GetAll".try_into().unwrap(),
        );

        msg.add_value(Value::String(interface.to_string()));

        msg
    }

    /// Create a [`Message`] to retrieve property value.
    pub fn property_set(
        destination: Bus,
        object_path: ObjectPath,
        interface: Interface,
        property: &str,
        value: Value,
    ) -> Message {
        let mut msg = Message::method_call(
            destination,
            object_path,
            "org.freedesktop.DBus.Properties".try_into().unwrap(),
            "Set".try_into().unwrap(),
        );

        msg.add_value(Value::String(interface.to_string()));
        msg.add_value(Value::String(property.to_string()));
        msg.add_value(Value::Variant(Box::new(value)));

        msg
    }

    /// Get the serial number.
    #[inline]
    pub const fn get_serial(&self) -> u32 {
        self.header.get_serial()
    }

    /// Set the serial number.
    #[inline]
    pub fn set_serial(&mut self, serial: u32) {
        self.header.serial = serial;
    }

    get_field!(
        /// Get the [`path`], if there is one in the header field.
        ///
        /// [`path`]: crate::message::MessageHeaderFields::path
        get_path,
        &ObjectPath
    );

    has_field!(
        /// It is true if the message contains a [`path`] in the header fields.
        ///
        /// [`path`]: crate::message::MessageHeaderField::path
        has_path
    );

    get_field!(
        /// Get the [`interface`], if there is one in the header field.
        ///
        /// [`interface`]: crate::message::MessageHeaderFields::interface
        get_interface,
        &Interface
    );

    has_field!(
        /// It is true if the message contains an [`interface`] in the header fields.
        ///
        /// [`interface`]: crate::message::MessageHeaderFields::interface
        has_interface
    );

    get_field!(
        /// Get the [`member`], if there is one in the header field.
        ///
        /// [`member`]: crate::message::MessageHeaderFields::member
        get_member,
        &Member
    );

    has_field!(
        /// It is true if the message contains a [`member`] in the header fields.
        ///
        /// [`member`]: crate::message::MessageHeaderFields::member
        has_member
    );

    get_field!(
        /// Get the [`error_name`], if there is one in the header field.
        ///
        /// [`error_name`]: crate::message::MessageHeaderFields::error_name
        get_error_name,
        &Error
    );

    has_field!(
        /// It is true if the message contains an [`error_name`] in the header fields.
        ///
        /// [`error_name`]: crate::message::MessageHeaderFields::error_name
        has_error_name
    );

    get_field!(
        /// Get the [`destination`], if there is one in the header field.
        ///
        /// [`destination`]: crate::message::MessageHeaderFields::destination
        get_destination,
        &Bus
    );

    has_field!(
        /// It is true if the message contains a [`destination`] in the header fields.
        ///
        /// [`destination`]: crate::message::MessageHeaderFields::destination
        has_destination
    );

    get_field!(
        /// Get the [`sender`], if there is one in the header field.
        ///
        /// [`sender`]: crate::message::MessageHeaderFields::sender
        get_sender,
        &Bus
    );

    has_field!(
        /// It is true if the message contains a [`sender`] in the header fields.
        ///
        /// [`sender`]: crate::message::MessageHeaderFields::sender
        has_sender
    );

    get_field!(
        /// Get the [`reply_serial`], if there is one in the header field.
        ///
        /// [`reply_serial`]: crate::message::MessageHeaderFields::reply_serial
        get_reply_serial,
        u32
    );

    has_field!(
        /// It is true if the message contains a [`reply_serial`] in the header fields.
        ///
        /// [`reply_serial`]: crate::message::MessageHeaderFields::reply_serial
        has_reply_serial
    );

    /// Get the [`signature`], if there is one in the header field.
    ///
    /// [`signature`]: crate::message::MessageHeaderFields::signature
    pub fn get_signature(&self) -> Result<Vec<Type>, TypeError> {
        let mut signature = Vec::new();
        for value in &self.body {
            let type_ = value.get_type()?;
            signature.push(type_);
        }

        Ok(signature)
    }

    has_field!(
        /// It is true if the message contains a [`signature`] in the header fields.
        ///
        /// [`signature`]: crate::message::MessageHeaderFields::signature
        has_signature
    );

    #[cfg(target_family = "unix")]
    get_field!(
        /// Get the [`unix_fds`], if there is one in the header field.
        ///
        /// [`unix_fds`]: crate::message::MessageHeaderFields::unix_fds
        get_unix_fds,
        u32
    );

    #[cfg(target_family = "unix")]
    has_field!(
        /// It is true if the message contains a [`unix_fds`] in the header fields.
        ///
        /// [`unix_fds`]: crate::message::MessageHeaderFields::unix_fds
        has_unix_fds
    );

    /// Add a new value to the body.
    pub fn add_value(&mut self, value: Value) {
        self.body.push(value);
    }

    /// Create a message return from this [`Message`].
    /// Only works if this [`Message`] is a [`MethodCall`].
    ///
    /// [`MethodCall`]: crate::message::MessageType::MethodCall
    pub fn method_return(&self) -> Result<Message, Message> {
        self.header.method_return()
    }

    /// Create a unknown property error message from this [`Message`].
    pub fn unknown_property(&self, property: &str) -> Message {
        self.header.unknown_property(property)
    }

    /// Create a unknown path error message from this [`Message`].
    pub fn unknown_path(&self) -> Option<Message> {
        self.header.unknown_path()
    }

    /// Create a unknown interface error message from this [`Message`].
    pub fn unknown_interface(&self) -> Option<Message> {
        self.header.unknown_interface()
    }

    /// Create a unknown member error message from this [`Message`].
    pub fn unknown_member(&self) -> Option<Message> {
        self.header.unknown_member()
    }

    /// Create an invalid args error message from this [`Message`].
    pub fn invalid_args(&self, reason: String) -> Message {
        self.header.invalid_args(reason)
    }

    /// Create an error message from this [`Message`].
    pub fn error(&self, name: Error, message: String) -> Message {
        self.header.error(name, message)
    }

    /// Get the body.
    #[inline]
    pub fn get_body(&self) -> &[Value] {
        &self.body
    }

    /// Get the message type.
    #[inline]
    pub fn get_type(&self) -> MessageType {
        self.header.get_type()
    }

    /// Split the [`Message`] object into the header and the body.
    pub fn split(mut self) -> Result<(Header, Vec<Value>), TypeError> {
        let signature = self.get_signature()?;
        if !signature.is_empty() {
            self.header.fields.signature = Some(signature);
        }
        Ok((self.header, self.body))
    }
}
