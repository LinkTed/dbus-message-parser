# dbus-message-parser
A library to encode and decode [DBus message](https://dbus.freedesktop.org/doc/dbus-specification.html).
[![Build Status](https://travis-ci.org/LinkTed/dbus-message-parser.svg?branch=master)](https://travis-ci.org/LinkTed/dbus-message-parser)
[![dependency status](https://deps.rs/repo/github/linkted/dbus-message-parser/status.svg)](https://deps.rs/repo/github/linkted/dbus-message-parser)
[![Latest version](https://img.shields.io/crates/v/dbus-message-parser.svg)](https://crates.io/crates/dbus-message-parser)
[![License](https://img.shields.io/crates/l/dbus-message-parser.svg)](https://opensource.org/licenses/LGPL-3.0)

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
dbus-message-parser = "1.0"
```

## Example
The following examples show how to create a `METHOD_CALL` message and a `SIGNAL`
 message.
```rust
use dbus_message_parser::{Message, Value};
use bytes::BytesMut;

fn create_method_call() {
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

fn create_signal() {
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
```