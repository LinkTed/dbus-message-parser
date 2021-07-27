# dbus-message-parser
A library to encode and decode [DBus message](https://dbus.freedesktop.org/doc/dbus-specification.html).
[![Build status](https://github.com/LinkTed/dbus-message-parser/workflows/Continuous%20Integration/badge.svg)](https://github.com/LinkTed/dbus-message-parser/actions?query=workflow%3A%22Continuous+Integration%22)
[![Code coverage](https://codecov.io/gh/LinkTed/dbus-message-parser/branch/master/graph/badge.svg)](https://codecov.io/gh/LinkTed/dbus-message-parser)
[![Latest version](https://img.shields.io/crates/v/dbus-message-parser.svg)](https://crates.io/crates/dbus-message-parser)
[![License](https://img.shields.io/crates/l/dbus-message-parser.svg)](https://opensource.org/licenses/LGPL-3.0)
[![Dependency status](https://deps.rs/repo/github/linkted/dbus-message-parser/status.svg)](https://deps.rs/repo/github/linkted/dbus-message-parser)

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
dbus-message-parser = "~4.2.0"
```

## Example
The following examples show how to create a `METHOD_CALL` message and a `SIGNAL` message.
```rust
use bytes::Bytes;
use dbus_message_parser::message::Message;
use dbus_message_parser::value::Value;
use std::convert::TryInto;

fn create_method_call() {
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

fn create_signal() {
    // Create a Signal
    // Arguments
    // 1. object path
    // 2. interface
    // 3. Signal name
    let mut signal = Message::signal(
        "/object/path".try_into().unwrap(),
        "interface.name".try_into.unwrap(),
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

fn decode_method_call() {
    // A message encoded as bytes
    let bytes = Bytes::copy_from_slice(
        &b"\x6c\x01\x00\x01\x0a\x00\x00\x00\xb1\x00\x00\x00\x9e\x00\x00\x00\x01\x01\x6f\x00\x15\x00\
        \x00\x00\x2f\x6f\x72\x67\x2f\x66\x72\x65\x65\x64\x65\x73\x6b\x74\x6f\x70\x2f\x44\x42\x75\
        \x73\x00\x00\x00\x02\x01\x73\x00\x14\x00\x00\x00\x6f\x72\x67\x2e\x66\x72\x65\x65\x64\x65\
        \x73\x6b\x74\x6f\x70\x2e\x44\x42\x75\x73\x00\x00\x00\x00\x06\x01\x73\x00\x14\x00\x00\x00\
        \x6f\x72\x67\x2e\x66\x72\x65\x65\x64\x65\x73\x6b\x74\x6f\x70\x2e\x44\x42\x75\x73\x00\x00\
        \x00\x00\x08\x01\x67\x00\x01\x73\x00\x00\x03\x01\x73\x00\x1a\x00\x00\x00\x47\x65\x74\x43\
        \x6f\x6e\x6e\x65\x63\x74\x69\x6f\x6e\x55\x6e\x69\x78\x50\x72\x6f\x63\x65\x73\x73\x49\x44\
        \x00\x00\x00\x00\x00\x00\x07\x01\x73\x00\x05\x00\x00\x00\x3a\x31\x2e\x35\x30\x00\x00\x00\
        \x05\x00\x00\x00\x3a\x31\x2e\x35\x35\x00"[..],
    );
    // Decode the message
    let (msg, _) = Message::decode(bytes).unwrap();
    println!("Message is decoded: {:?}", msg);
}
```

## License
This project is licensed under the [LGPL-3.0](https://opensource.org/licenses/LGPL-3.0) license.

### Contribution
Any contribution intentionally submitted for inclusion in `dbus-message-parser` by you, shall be 
licensed as [LGPL-3.0](https://opensource.org/licenses/LGPL-3.0), without any additional terms or 
conditions.
