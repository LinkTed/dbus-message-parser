use bytes::BytesMut;
use dbus_message_parser::{EncodeError, Encoder, Value};

#[test]
fn array_signature_mismatch() {
    let int_32 = Value::Int32(10);
    let int_16 = Value::Int16(10);
    let vec = vec![int_32, int_16];
    let sig = "i".to_string();

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
    assert_eq!(
        encoder.array(&vec, &sig, true),
        Err(EncodeError::ArraySignatureMismatch(
            "i".to_string(),
            "n".to_string(),
        )),
    );
}

#[test]
fn array_unknown_signature() {
    let vec = vec![];
    let sig = "l".to_string();

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
    assert_eq!(
        encoder.array(&vec, &sig, true),
        Err(EncodeError::UnknownSignature("l".to_string())),
    );
}

#[test]
fn array_null_signature() {
    let vec = vec![];
    let sig = "".to_string();

    let mut buffer = BytesMut::new();
    #[cfg(target_family = "unix")]
    let mut fds = Vec::new();
    #[cfg(target_family = "unix")]
    let mut encoder = Encoder::new(&mut buffer, &mut fds);
    #[cfg(not(target_family = "unix"))]
    let mut encoder = Encoder::new(&mut buffer);
    assert_eq!(
        encoder.array(&vec, &sig, true),
        Err(EncodeError::NullSignature),
    );
}
