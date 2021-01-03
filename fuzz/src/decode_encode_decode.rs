#[macro_use]
extern crate honggfuzz;
use bytes::Bytes;
use dbus_message_parser::message::Message;
use std::cmp::Ordering;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let bytes = Bytes::copy_from_slice(data);
            if let Ok((msg_1, _)) = Message::decode(bytes) {
                let bytes = msg_1.encode().unwrap();
                let bytes = bytes.freeze();
                match Message::decode(bytes.clone()) {
                    Ok((msg_2, _)) => {
                        if let Some(ordering) = msg_1.partial_cmp(&msg_2) {
                            match ordering {
                                Ordering::Equal => {}
                                _ => {
                                    panic!(
                                        "Messsage is not equal: {:?} != {:?}\n{:?} != {:?}",
                                        msg_1, msg_2, data, bytes,
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        panic!(
                            "Could not decode DBus message:\n{:?}\n{:?}\n{:?}\n{:?}",
                            e, bytes, data, msg_1
                        );
                    }
                }
            }
        });
    }
}
