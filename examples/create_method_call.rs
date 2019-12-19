use dbus_message_parser::{Message, Value};
use bytes::BytesMut;


fn main() {
    // Create a MessageCall
    // Arguments:
    // 1. destination
    // 2. object path
    // 3. interface
    // 4. method
    let mut msg = Message::method_call("destination.address",
                                       "/object/path",
                                       "interface.name",
                                       "MethodName");

    // Add the first argument to the MessageCall
    msg.add_value(Value::String("String Argument".to_string()));
    // Add the second argument to the MessageCall
    msg.add_value(Value::Uint32(0));

    println!("{:?}", msg);

    let mut buffer = BytesMut::new();
    msg.encode(&mut buffer).unwrap();

    println!("{:?}", buffer);
}