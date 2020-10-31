use bytes::BytesMut;
use dbus_message_parser::{Encoder, Message, Value};
use std::convert::TryInto;

fn main() {
    // Create a Signal
    // Arguments
    // 1. object path
    // 2. interface
    // 3. Signal name
    let mut signal = Message::signal(
        "/object/path".try_into().unwrap(),
        "interface.name",
        "SignalName",
    );

    // Add the first argument to the MessageCall
    signal.add_value(Value::Uint32(0));
    // Add the second argument to the MessageCall
    signal.add_value(Value::Double(1.0));

    println!("{:?}", signal);

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
    encoder.message(&signal).unwrap();

    println!("{:?}", buffer);
}
