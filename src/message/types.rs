use std::convert::TryFrom;

/// An enum representing the [message type].
///
/// [message type]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types
#[repr(u8)]
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub enum MessageType {
    /// The message is a [`METHOD_CALL`].
    ///
    /// [`METHOD_CALL`]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types-method
    MethodCall = 1,
    /// The message is a [`METHOD_RETURN`].
    ///
    /// [`METHOD_RETURN`]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types-method
    MethodReturn = 2,
    /// The message is a [`ERROR`].
    ///
    /// [`ERROR`]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types-errors
    Error = 3,
    /// The message is a [`SIGNAL`].
    ///
    /// [`SIGNAL`]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types-signal
    Signal = 4,
}

impl TryFrom<u8> for MessageType {
    type Error = u8;

    fn try_from(message_type: u8) -> Result<MessageType, u8> {
        match message_type {
            1 => Ok(MessageType::MethodCall),
            2 => Ok(MessageType::MethodReturn),
            3 => Ok(MessageType::Error),
            4 => Ok(MessageType::Signal),
            message_type => Err(message_type),
        }
    }
}
