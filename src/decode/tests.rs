use crate::decode::{DecodeError, Decoder};
use bytes::Bytes;

#[test]
fn algin_padding_error() {
    let b = Bytes::from_static(b"\x01\x01");
    let mut decoder = Decoder::new(b);
    decoder.offset = 1;
    assert_eq!(decoder.algin(2), Err(DecodeError::Padding(0x01)));
}

#[test]
fn algin_not_engough_bytes_error() {
    let b = Bytes::from_static(b"\x01");
    let mut decoder = Decoder::new(b);
    decoder.offset = 1;
    assert_eq!(decoder.algin(2), Err(DecodeError::NotEnoughBytes(1, 2)));
}

#[test]
fn check_add_integer_overflow() {
    let u = Decoder::<'static>::checked_add(std::usize::MAX, 1);
    assert_eq!(u, Err(DecodeError::IntegerOverflow(std::usize::MAX, 1)));
}
