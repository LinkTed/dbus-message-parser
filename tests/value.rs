use dbus_message_parser::Value;

#[test]
fn value() {
    // Create two uint32 value
    let uint32_100 = Value::Uint32(100);
    let uint32_20 = Value::Uint32(20);

    // Create a string value
    let string = Value::String(String::from("DBus String"));

    if let Value::String(string_value) = &string {
        // print value
        println!("{}", string_value);
    }

    // Create an array of uint32
    let _array = Value::Array(vec![uint32_100, uint32_20], String::from("u"));
}
