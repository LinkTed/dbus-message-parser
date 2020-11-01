use bytes::BytesMut;
use dbus_message_parser::{Encoder, Message, Value};
use std::convert::TryInto;

#[test]
fn method_call() {
    let mut msg = Message::method_call(
        "destination.address".try_into().unwrap(),
        "/object/path".try_into().unwrap(),
        "interface.name".try_into().unwrap(),
        "MethodName".try_into().unwrap(),
    );
    msg.add_value(Value::String("String Argument".to_string()));
    msg.add_value(Value::Uint32(0));

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
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
fn signal() {
    let mut signal = Message::signal(
        "/object/path".try_into().unwrap(),
        "interface.name".try_into().unwrap(),
        "SignalName".try_into().unwrap(),
    );
    signal.add_value(Value::Uint32(0));
    signal.add_value(Value::Double(1.0));

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
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
