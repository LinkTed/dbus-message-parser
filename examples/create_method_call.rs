use dbus_message_parser::message::Message;
use dbus_message_parser::value::Value;
use std::convert::TryInto;

fn main() {
    // Create a MessageCall
    // Arguments:
    // 1. destination
    // 2. object path
    // 3. interface
    // 4. method
    let mut msg = Message::method_call(
        "destination.address".try_into().unwrap(),
        "/object/path".try_into().unwrap(),
        "interface.name".try_into().unwrap(),
        "MethodName".try_into().unwrap(),
    );

    // Add the first argument to the MessageCall
    msg.add_value(Value::String("String Argument".to_string()));
    // Add the second argument to the MessageCall
    msg.add_value(Value::Uint32(0));

    println!("{:?}", msg);

    let bytes = msg.encode().unwrap();

    println!("{:?}", bytes);
}
