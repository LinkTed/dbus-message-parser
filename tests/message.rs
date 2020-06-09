use bytes::{Bytes, BytesMut};
use dbus_message_parser::{Decoder, Encoder, Message, MessageType, Value};

fn decode_encode(msg: &[u8]) {
    // Decode Bytes to Message
    let bytes = Bytes::copy_from_slice(&msg[..]);
    let mut decoder = Decoder::new(&bytes);
    let msg = decoder.message().unwrap();

    // Encode Message to BytesMut
    let mut bytes = BytesMut::new();
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut bytes, &mut fds);
    encoder.message(&msg).unwrap();
}

#[test]
fn msg_1_encode_decode() {
    let msg = b"l\x02\x01\x01\n\0\0\0\x01\0\0\0=\0\0\0\x06\x01s\0\x05\0\0\0\
    :1.98\0\0\0\x05\x01u\0\x01\0\0\0\x08\x01g\0\x01s\0\0\x07\x01s\0\x14\0\0\0\
    org.freedesktop.DBus\0\0\0\0\x05\0\0\0:1.98\0";

    decode_encode(&msg[..]);
}

#[test]
fn msg_2_encode_decode() {
    let msg = b"l\x02\x01\x01\xec\x00\x00\x00`\x00\x00\x006\x00\x00\x00\x06\
    \x01s\x00\x06\x00\x00\x00:1.105\x00\x00\x08\x01g\x00\na{s(bgav)}\x00\x05\
    \x01u\x009\x01\x00\x00\x07\x01s\x00\x05\x00\x00\x00:1.99\x00\x00\x00\xe4\
    \x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00quit\x00\x00\x00\x00\x00\x00\
    \x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x0c\x00\x00\x00new-document\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x00\x00\
    preferences\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\t\x00\x00\x00shortcuts\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00help\x00\x00\x00\x00\
    \x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x05\x00\x00\x00about\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\n\x00\x00\x00new-window\
    \x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    decode_encode(&msg[..]);
}

#[test]
fn msg_3_encode_decode() {
    let msg = b"l\x04\x01\x01t\x00\x00\x00\xb8\x00\x00\x00v\x00\x00\x00\x01\
    \x01o\x00\x10\x00\x00\x00/org/gnome/dfeet\x00\x00\x00\x00\x00\x00\x00\x00\
    \x02\x01s\x00\x0f\x00\x00\x00org.gtk.Actions\x00\x08\x01g\x00\x16\
    asa{sb}a{sv}a{s(bgav)}\x00\x00\x00\x00\x00\x03\x01s\x00\x07\x00\x00\x00\
    Changed\x00\x07\x01s\x00\x05\x00\x00\x00:1.89\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\\\x00\x00\x00\x00\x00\x00\
    \x00\x04\x00\x00\x00help\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x00about\x00\
    \x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x04\x00\x00\x00quit\x00\x00\x00\x00\x00\x00\x00\x00\x01\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    decode_encode(&msg[..]);
}

#[test]
fn msg_4_encode_decode() {
    let msg = b"l\x01\x00\x01\x00\x00\x00\x00\xbd\x00\x00\x00\x8e\x00\x00\x00\
    \x01\x01o\x00\x04\x00\x00\x00/org\x00\x00\x00\x00\x02\x01s\x00#\x00\x00\
    \x00org.freedesktop.DBus.Introspectable\x00\x00\x00\x00\x00\x06\x01s\x00\
    \x1c\x00\x00\x00org.freedesktop.FileManager1\x00\x00\x00\x00\x03\x01s\x00\
    \n\x00\x00\x00Introspect\x00\x00\x00\x00\x00\x00\x07\x01s\x00\x05\x00\x00\
    \x00:1.89\x00\x00\x00";

    decode_encode(&msg[..]);
}

#[test]
fn method_call_encode() {
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    msg.add_value(Value::String("String Argument".to_string()));
    msg.add_value(Value::Uint32(0));

    let mut buffer = BytesMut::new();
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    encoder.message(&msg).expect("Try to encode message");
    assert_eq!(
        &buffer,
        &b"\x6c\x01\x00\x01\x18\x00\x00\x00\x00\x00\x00\x00\x70\x00\x00\x00\x01\x01\x6f\x00\x0c\x00\
        \x00\x00\x2f\x6f\x62\x6a\x65\x63\x74\x2f\x70\x61\x74\x68\x00\x00\x00\x00\x02\x01\x73\x00\
        \x0e\x00\x00\x00\x69\x6e\x74\x65\x72\x66\x61\x63\x65\x2e\x6e\x61\x6d\x65\x00\x00\x03\x01\
        \x73\x00\x0a\x00\x00\x00\x4d\x65\x74\x68\x6f\x64\x4e\x61\x6d\x65\x00\x00\x00\x00\x00\x00\
        \x06\x01\x73\x00\x13\x00\x00\x00\x64\x65\x73\x74\x69\x6e\x61\x74\x69\x6f\x6e\x2e\x61\x64\
        \x64\x72\x65\x73\x73\x00\x00\x00\x00\x00\x08\x01\x67\x00\x02\x73\x75\x00\x0f\x00\x00\x00\
        \x53\x74\x72\x69\x6e\x67\x20\x41\x72\x67\x75\x6d\x65\x6e\x74\x00\x00\x00\x00\x00"[..]
    );
}

#[test]
fn get_serial() {
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    msg.set_serial(443);
    assert_eq!(msg.get_serial(), 443);
}

#[test]
fn get_reply_serial() {
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    msg.set_serial(443);
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_reply_serial(), Some(443));
}

#[test]
fn get_reply_serial_none() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_reply_serial(), None);
}

#[test]
fn get_path() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_path(), Some("/object/path"));
}

#[test]
fn get_path_none() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_path(), None);
}

#[test]
fn has_interface() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert!(msg.has_interface());
}

#[test]
fn get_interface() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_interface(), Some("interface.name"));
}

#[test]
fn get_interface_none() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_interface(), None);
}

#[test]
fn has_member() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert!(msg.has_member());
}

#[test]
fn get_member() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_member(), Some("MethodName"));
}

#[test]
fn get_member_none() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_member(), None);
}

#[test]
fn has_error_name() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert!(!msg.has_error_name());
}

#[test]
fn get_error_name() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let msg = msg.error("name".to_string(), "message".to_string());
    assert_eq!(msg.get_error_name(), Some("name"));
}

#[test]
fn get_sender() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_sender(), Some("destination.address"));
}

#[test]
fn get_sender_none() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_sender(), None);
}

#[test]
fn get_destination() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_destination(), Some("destination.address"));
}

#[test]
fn get_destination_none() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_destination(), None);
}

#[test]
fn get_signature() {
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    msg.add_value(Value::Uint32(0));
    assert_eq!(msg.get_signature(), "u");
}

#[test]
fn get_body() {
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    msg.add_value(Value::Uint32(0));
    assert_eq!(msg.get_body(), &[Value::Uint32(0)][..]);
}

#[test]
fn get_type() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    assert_eq!(msg.get_type(), MessageType::MethodCall);
}

#[test]
fn split() {
    let mut msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    msg.add_value(Value::Uint32(0));
    let (header, body) = msg.split();
    assert_eq!(header.get_signature(), Some("u"));
    assert_eq!(body, &[Value::Uint32(0)][..]);
}

#[test]
fn has_signature() {
    let msg = Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    );
    let (header, _) = msg.split();
    assert!(!header.has_signature());
}

#[test]
fn signal_encode() {
    let mut signal = Message::signal("/object/path", "interface.name", "SignalName");
    signal.add_value(Value::Uint32(0));
    signal.add_value(Value::Double(1.0));

    let mut buffer = BytesMut::new();
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    encoder.message(&signal).expect("Try to encode message");
    assert_eq!(
        &buffer,
        &b"\x6c\x04\x01\x01\x10\x00\x00\x00\x00\x00\x00\x00\x50\x00\x00\x00\x01\x01\x6f\x00\x0c\x00\
        \x00\x00\x2f\x6f\x62\x6a\x65\x63\x74\x2f\x70\x61\x74\x68\x00\x00\x00\x00\x02\x01\x73\x00\
        \x0e\x00\x00\x00\x69\x6e\x74\x65\x72\x66\x61\x63\x65\x2e\x6e\x61\x6d\x65\x00\x00\x03\x01\
        \x73\x00\x0a\x00\x00\x00\x53\x69\x67\x6e\x61\x6c\x4e\x61\x6d\x65\x00\x00\x00\x00\x00\x00\
        \x08\x01\x67\x00\x02\x75\x64\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
        \xf0\x3f"[..],
    );
}
