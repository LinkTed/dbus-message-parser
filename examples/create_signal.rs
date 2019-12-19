use dbus_message_parser::{Message, Value};
use bytes::BytesMut;


fn main() {
    // Create a Signal
    // Arguments
    // 1. object path
    // 2. interface
    // 3. Signal name
    let mut signal = Message::signal("/object/path",
                                     "interface.name",
                                     "SignalName");

    // Add the first argument to the MessageCall
    signal.add_value(Value::Uint32(0));
    // Add the second argument to the MessageCall
    signal.add_value(Value::Double(1.0));

    println!("{:?}", signal);

    let mut buffer = BytesMut::new();
    signal.encode(&mut buffer).unwrap();

    println!("{:?}", buffer);
}