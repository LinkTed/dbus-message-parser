use dbus_message_parser::message::Message;
use dbus_message_parser::value::Value;
use std::convert::TryInto;

fn main() {
    // Create a Signal
    // Arguments
    // 1. object path
    // 2. interface
    // 3. Signal name
    let mut signal = Message::signal(
        "/object/path".try_into().unwrap(),
        "interface.name".try_into().unwrap(),
        "SignalName".try_into().unwrap(),
    );

    // Add the first argument to the MessageCall
    signal.add_value(Value::Uint32(0));
    // Add the second argument to the MessageCall
    signal.add_value(Value::Double(1.0));

    println!("{:?}", signal);

    let bytes = signal.encode().unwrap();

    println!("{:?}", bytes);
}
