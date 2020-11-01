use bytes::BytesMut;
use dbus_message_parser::{Encoder, Message, Value};
use std::convert::TryInto;

fn main() {
    // Create a MessageCall
    // Arguments:
    // 1. destination
    // 2. object path
    // 3. interface
    // 4. method
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path".try_into().unwrap(),
        "interface.name",
        "MethodName".try_into().unwrap(),
    );

    // Add the first argument to the MessageCall
    msg.add_value(Value::String("String Argument".to_string()));
    // Add the second argument to the MessageCall
    msg.add_value(Value::Uint32(0));

    println!("{:?}", msg);

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
    encoder.message(&msg).unwrap();

    println!("{:?}", buffer);
}
