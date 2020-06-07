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
