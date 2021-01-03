use crate::encode::{EncodeError, Encoder};
use crate::value::Value;
use std::convert::TryInto;

#[test]
fn array_signature_mismatch() {
    let int_32 = Value::Int32(10);
    let int_16 = Value::Int16(10);
    let vec = vec![int_32, int_16];
    let sig = "i".try_into().unwrap();

    let mut encoder = Encoder::new();
    assert_eq!(
        encoder.array(&vec, &sig, true),
        Err(EncodeError::ArraySignatureMismatch(
            "i".try_into().unwrap(),
            "n".try_into().unwrap(),
        )),
    );
}
