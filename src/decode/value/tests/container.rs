use crate::decode::{DecodeError, Decoder};
use crate::value::Signature;
use crate::value::Value;
use bytes::Bytes;
use std::convert::TryFrom;

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
    let byte = Value::Byte(0x01);
    let v = vec![byte.clone(), byte];
    assert_eq!(
        decoder.variant(true, 0),
        Err(DecodeError::VariantSingleValue(v))
    );
}

#[test]
fn array_maximum_length_error() {
    let b = Bytes::from_static(b"\xff\xff\xff\xff");
    let mut decoder = Decoder::new(b);
    let signature = Signature::try_from("y").unwrap();
    assert_eq!(
        decoder.d_array(true, 0, &signature),
        Err(DecodeError::ArrayTooBig(0xffffffff))
    );
}

#[test]
fn array_signature_emtpy_error() {
    let b = Bytes::from_static(b"\x01\x00\x00\x00\x01");
    let mut decoder = Decoder::new(b);
    let signature = Signature::try_from("").unwrap();
    assert_eq!(
        decoder.d_array(true, 0, &signature),
        Err(DecodeError::ArraySignatureEmpty)
    );
}

#[test]
fn array_single_value_error() {
    let b = Bytes::from_static(b"\x02\x00\x00\x00\x01\x01");
    let mut decoder = Decoder::new(b);
    let signature = Signature::try_from("yy").unwrap();
    let byte = Value::Byte(0x01);
    let v = vec![byte.clone(), byte];
    assert_eq!(
        decoder.d_array(true, 0, &signature),
        Err(DecodeError::ArraySingleValue(v))
    );
}

#[test]
fn array_invalid_length_error() {
    let b = Bytes::from_static(b"\x01\x00\x00\x00\x01\x00\x00\x00");
    let mut decoder = Decoder::new(b);
    let signature = Signature::try_from("i").unwrap();
    assert_eq!(
        decoder.d_array(true, 0, &signature),
        Err(DecodeError::ArrayInvalidLength(8, 5))
    );
}

#[test]
fn dict_entry_key_single_value_error() {
    let b = Bytes::from_static(b"\x01\x02\x01");
    let mut decoder = Decoder::new(b);
    let signature_key = Signature::try_from("yy").unwrap();
    let signature_value = Signature::try_from("y").unwrap();
    let byte_1 = Value::Byte(0x01);
    let byte_2 = Value::Byte(0x02);
    let v = vec![byte_1, byte_2];
    assert_eq!(
        decoder.dict_entry(true, 0, &signature_key, &signature_value),
        Err(DecodeError::DictKeySingleValue(v))
    );
}

#[test]
fn dict_entry_value_single_value_error() {
    let b = Bytes::from_static(b"\x01\x01\x02");
    let mut decoder = Decoder::new(b);
    let signature_key = Signature::try_from("y").unwrap();
    let signature_value = Signature::try_from("yy").unwrap();
    let byte_1 = Value::Byte(0x01);
    let byte_2 = Value::Byte(0x02);
    let v = vec![byte_1, byte_2];
    assert_eq!(
        decoder.dict_entry(true, 0, &signature_key, &signature_value),
        Err(DecodeError::DictValueSingleValue(v))
    );
}
