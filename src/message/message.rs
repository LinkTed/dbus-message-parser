use crate::{
    Bus, Error, Interface, Member, MessageFlags, MessageHeader, MessageHeaderField, MessageType,
    ObjectPath, Value,
};
use std::collections::BTreeSet;
use std::convert::TryInto;

/// This represents a DBus [message].
///
/// [message]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Message {
    pub(super) header: MessageHeader,
    pub(super) body: Vec<Value>,
}

impl Message {
    /// Create a `Message` object.
    pub fn new(header: MessageHeader, body: Vec<Value>) -> Message {
        Message { header, body }
    }

    /// Create a `Message` object as a MethodCall.
    pub fn method_call(
        destination: Bus,
        object_path: ObjectPath,
        interface: Interface,
        member: Member,
    ) -> Message {
        let mut fields = BTreeSet::new();

        fields.insert(MessageHeaderField::Destination(destination));
        fields.insert(MessageHeaderField::Path(object_path));
        fields.insert(MessageHeaderField::Interface(interface));
        fields.insert(MessageHeaderField::Member(member));

        let header = MessageHeader {
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

    /// Create a `Message` object as a Signal.
    pub fn signal(object_path: ObjectPath, interface: Interface, member: Member) -> Message {
        let mut fields = BTreeSet::new();

        fields.insert(MessageHeaderField::Path(object_path));
        fields.insert(MessageHeaderField::Interface(interface));
        fields.insert(MessageHeaderField::Member(member));

        let header = MessageHeader {
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

    /// Create a `Message` to retrieve property value.
    pub fn property_get(
        destination: Bus,
        object_path: ObjectPath,
        interface: &str,
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

    /// Create a `Message` to retrieve property value.
    pub fn properties_get_all(
        destination: Bus,
        object_path: ObjectPath,
        interface: &str,
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

    /// Create a `Message` to retrieve property value.
    pub fn property_set(
        destination: Bus,
        object_path: ObjectPath,
        interface: &str,
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
    pub fn get_path(&self) -> Option<&ObjectPath> {
        self.header.get_path()
    }

    /// It is true if the message contains an `Interface` in the header fields.
    #[inline]
    pub fn has_interface(&self) -> bool {
        self.header.has_interface()
    }

    /// Get the `Interface`, if there is one in the header fields.
    pub fn get_interface(&self) -> Option<&Interface> {
        self.header.get_interface()
    }

    /// It is true if the message contains an `Member` in the header fields.
    #[inline]
    pub fn has_member(&self) -> bool {
        self.header.has_member()
    }

    /// Get the `Member`, if there is one in the header field.
    pub fn get_member(&self) -> Option<&Member> {
        self.header.get_member()
    }

    /// It is true if the message contains an `ErrorName` in the header fields.
    #[inline]
    pub fn has_error_name(&self) -> bool {
        self.header.has_error_name()
    }

    /// Get the `ErrorName`, if there is one in the header field.
    pub fn get_error_name(&self) -> Option<&Error> {
        self.header.get_error_name()
    }

    /// Get the `Sender`, if there is one in the header field.
    pub fn get_sender(&self) -> Option<&Bus> {
        self.header.get_sender()
    }

    /// Get the `Destination`, if there is one in the header field.
    pub fn get_destination(&self) -> Option<&Bus> {
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

    /// Get the `UnixFDs`, if there is one in the header field.
    #[cfg(target_family = "unix")]
    pub fn get_unix_fds(&self) -> Option<u32> {
        self.header.get_unix_fds()
    }

    /// It is true if the message contains an `UnixFDs` in the header fields.
    #[cfg(target_family = "unix")]
    #[inline]
    pub fn has_unix_fds(&self) -> bool {
        self.header.has_unix_fds()
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
    pub fn invalid_args(&self, reason: String) -> Message {
        self.header.invalid_args(reason)
    }

    /// Create an error message from this `Message`.
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

    /// Split the `Message` object into the header and the body.
    pub fn split(mut self) -> (MessageHeader, Vec<Value>) {
        let signature = self.get_signature();
        if !signature.is_empty() {
            self.header
                .fields
                .insert(MessageHeaderField::Signature(signature));
        }
        (self.header, self.body)
    }
}
