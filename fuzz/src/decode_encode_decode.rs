#[macro_use]
extern crate afl;
use bytes::BytesMut;
use dbus_message_parser::{Decoder, Encoder};
use std::cmp::Ordering;

fn main() {
    fuzz!(|data: &[u8]| {
        let bytes = Vec::from(data);
        let mut decoder = Decoder::new(&bytes);
        if let Ok(msg_1) = decoder.message() {
            let mut fds = Vec::new();
            let mut bytes = BytesMut::default();
            let mut encoder = Encoder::new(&mut bytes, &mut fds);
            if let Err(e) = encoder.message(&msg_1) {
                panic!("Could not encode DBus message: {:?}: {:?}", e, data);
            }
            let mut decoder = Decoder::new(&bytes);
            match decoder.message() {
                Ok(msg_2) => {
                    if let Some(ordering) = msg_1.partial_cmp(&msg_2) {
                        match ordering {
                            Ordering::Equal => {}
                            _ => {
                                panic!(
                                    "Messsage is not equal: {:?} != {:?}: {:?}",
                                    msg_1, msg_2, data
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!(
                        "Could not decode DBus message: {:?}: {:?}: {:?}",
                        e, bytes, data
                    );
                }
            }
        }
    });
}
