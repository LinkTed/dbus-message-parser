use crate::decode::{DecodeError, Decoder};
use crate::value::{Type, TypeError};
use bytes::Bytes;

#[test]
fn variant_depth_error() {
    let b = Bytes::from_static(b"\x01\x76\x00\x01\x76\x00\x01\x76\x00\x01\x76\x00");
    let mut decoder = Decoder::new(b);
    assert_eq!(decoder.variant(true, 0), Err(DecodeError::VariantDepth(5)));
}

#[test]
fn variant_single_value_error() {
    let b = Bytes::from_static(b"\x02\x79\x79\x00\x01\x01");
    let mut decoder = Decoder::new(b);
    assert_eq!(
        decoder.variant(true, 0),
        Err(DecodeError::SignatureError(TypeError::MultiplyTypes))
    );
}

#[test]
fn array_maximum_length_error() {
    let b = Bytes::from_static(b"\xff\xff\xff\xff");
    let mut decoder = Decoder::new(b);
    let type_ = Type::Byte;
    assert_eq!(
        decoder.d_array(true, 0, &type_),
        Err(DecodeError::ArrayTooBig(0xffffffff))
    );
}

#[test]
fn array_invalid_length_error() {
    let b = Bytes::from_static(b"\x01\x00\x00\x00\x01\x00\x00\x00");
    let mut decoder = Decoder::new(b);
    let type_ = Type::Int32;
    assert_eq!(
        decoder.d_array(true, 0, &type_),
        Err(DecodeError::ArrayInvalidLength(8, 5))
    );
}
