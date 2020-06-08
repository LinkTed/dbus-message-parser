/// An enum representing the [message type].
///
/// [message type]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-types
#[derive(Debug, PartialEq, Eq, Clone, FromPrimitive, ToPrimitive)]
pub enum MessageType {
    MethodCall = 1,
    MethodReturn = 2,
    Error = 3,
    Signal = 4,
}
