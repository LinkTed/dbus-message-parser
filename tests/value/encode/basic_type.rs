use bytes::BytesMut;
use dbus_message_parser::{EncodeError, Encoder, Value};
use std::convert::TryInto;

macro_rules! init_test {
    ($array:tt, $value:expr, $le:expr) => {{
        let mut b = BytesMut::from(&$array[..]);
        #[cfg(target_family = "unix")]
        let mut fds = Vec::new();
        #[cfg(target_family = "unix")]
        let mut encoder = Encoder::new(&mut b, &mut fds);
        #[cfg(not(target_family = "unix"))]
        let mut encoder = Encoder::new(&mut b);
        let v = $value;
        encoder.value(&v, $le).unwrap();
        b
    }};
}

macro_rules! init_error_test {
    ($array:tt, $value:expr, $le:expr) => {{
        let mut b = BytesMut::from(&$array[..]);
        #[cfg(target_family = "unix")]
        let mut fds = Vec::new();
        #[cfg(target_family = "unix")]
        let mut encoder = Encoder::new(&mut b, &mut fds);
        #[cfg(not(target_family = "unix"))]
        let mut encoder = Encoder::new(&mut b);
        let v = $value;
        encoder.value(&v, $le)
    }};
}

macro_rules! end_test {
    ($b:ident, $array:tt) => {
        assert_eq!(&$b[..], &$array[..]);
    };
}

#[test]
fn byte() {
    let b = init_test!(b"", Value::Byte(1), true);
    end_test!(b, b"\x01");
}

#[test]
fn boolean_1() {
    let b = init_test!(b"", Value::Boolean(false), false);
    end_test!(b, b"\x00\x00\x00\x00");
}

#[test]
fn boolean_2() {
    let b = init_test!(b"", Value::Boolean(true), true);
    end_test!(b, b"\x01\x00\x00\x00");
}

#[test]
fn boolean_3() {
    let b = init_test!(b"\x00", Value::Boolean(true), true);
    end_test!(b, b"\x00\x00\x00\x00\x01\x00\x00\x00");
}

#[test]
fn boolean_4() {
    let b = init_test!(b"", Value::Boolean(true), false);
    end_test!(b, b"\x00\x00\x00\x01");
}

#[test]
fn int_16_1() {
    let b = init_test!(b"", Value::Int16(-4095), true);
    end_test!(b, b"\x01\xf0");
}

#[test]
fn int_16_2() {
    let b = init_test!(b"", Value::Int16(-4095), false);
    end_test!(b, b"\xf0\x01");
}

#[test]
fn int_16_3() {
    let b = init_test!(b"\x00", Value::Int16(-4095), true);
    end_test!(b, b"\x00\x00\x01\xf0");
}

#[test]
fn uint_16_1() {
    let b = init_test!(b"", Value::Uint16(61441), true);
    end_test!(b, b"\x01\xf0");
}

#[test]
fn uint_16_2() {
    let b = init_test!(b"", Value::Uint16(61441), false);
    end_test!(b, b"\xf0\x01");
}

#[test]
fn uint_16_3() {
    let b = init_test!(b"\x00", Value::Uint16(61441), true);
    end_test!(b, b"\x00\x00\x01\xf0");
}

#[test]
fn int_32_1() {
    let b = init_test!(b"", Value::Int32(-268369663), true);
    end_test!(b, b"\x01\x01\x01\xf0");
}

#[test]
fn int_32_2() {
    let b = init_test!(b"", Value::Int32(-268369663), false);
    end_test!(b, b"\xf0\x01\x01\x01");
}

#[test]
fn int_32_3() {
    let b = init_test!(b"\x00", Value::Int32(-268369663), true);
    end_test!(b, b"\x00\x00\x00\x00\x01\x01\x01\xf0");
}

#[test]
fn uint_32_1() {
    let b = init_test!(b"", Value::Uint32(4026597633), true);
    end_test!(b, b"\x01\x01\x01\xf0");
}

#[test]
fn uint_32_2() {
    let b = init_test!(b"", Value::Uint32(4026597633), false);
    end_test!(b, b"\xf0\x01\x01\x01");
}

#[test]
fn uint_32_3() {
    let b = init_test!(b"\x00", Value::Uint32(4026597633), true);
    end_test!(b, b"\x00\x00\x00\x00\x01\x01\x01\xf0");
}

#[test]
fn int_64_1() {
    let b = init_test!(b"", Value::Int64(-1152638925806698239), true);
    end_test!(b, b"\x01\x01\x01\x01\x01\x01\x01\xf0");
}

#[test]
fn int_64_2() {
    let b = init_test!(b"", Value::Int64(-1152638925806698239), false);
    end_test!(b, b"\xf0\x01\x01\x01\x01\x01\x01\x01");
}

#[test]
fn int_64_3() {
    let b = init_test!(b"\x00", Value::Int64(-1152638925806698239), true);
    end_test!(
        b,
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x01\xf0"
    );
}

#[test]
fn uint_64_1() {
    let b = init_test!(b"", Value::Uint64(17294105147902853377), true);
    end_test!(b, b"\x01\x01\x01\x01\x01\x01\x01\xf0");
}

#[test]
fn uint_64_2() {
    let b = init_test!(b"", Value::Uint64(17294105147902853377), false);
    end_test!(b, b"\xf0\x01\x01\x01\x01\x01\x01\x01");
}

#[test]
fn uint_64_3() {
    let b = init_test!(b"\x00", Value::Uint64(17294105147902853377), true);
    end_test!(
        b,
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x01\xf0"
    );
}

#[test]
fn double_1() {
    let b = init_test!(b"", Value::Double(-123.456), true);
    end_test!(b, b"\x77\xbe\x9f\x1a\x2f\xdd\x5e\xc0");
}

#[test]
fn double_2() {
    let b = init_test!(b"", Value::Double(-123.456), false);
    end_test!(b, b"\xc0\x5e\xdd\x2f\x1a\x9f\xbe\x77");
}

#[test]
fn double_3() {
    let b = init_test!(b"\x00", Value::Double(-123.456), true);
    end_test!(
        b,
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x77\xbe\x9f\x1a\x2f\xdd\x5e\xc0"
    );
}

#[test]
fn string_1() {
    let b = init_test!(b"", Value::String(String::from("foo")), true);
    end_test!(b, b"\x03\x00\x00\x00\x66\x6f\x6f\x00");
}

#[test]
fn string_2() {
    let b = init_test!(b"", Value::String(String::from("foo")), false);
    end_test!(b, b"\x00\x00\x00\x03\x66\x6f\x6f\x00");
}

#[test]
fn string_3() {
    let b = init_test!(b"\x00", Value::String(String::from("foo")), true);
    end_test!(b, b"\x00\x00\x00\x00\x03\x00\x00\x00\x66\x6f\x6f\x00");
}

#[test]
fn path_1() {
    let b = init_test!(b"", Value::ObjectPath("/test".try_into().unwrap()), true);
    end_test!(b, b"\x05\x00\x00\x00\x2f\x74\x65\x73\x74\x00");
}

#[test]
fn path_2() {
    let b = init_test!(b"", Value::ObjectPath("/test".try_into().unwrap()), false);
    end_test!(b, b"\x00\x00\x00\x05\x2f\x74\x65\x73\x74\x00");
}

#[test]
fn path_3() {
    let b = init_test!(
        b"\x00",
        Value::ObjectPath("/test".try_into().unwrap()),
        true
    );
    end_test!(
        b,
        b"\x00\x00\x00\x00\x05\x00\x00\x00\x2f\x74\x65\x73\x74\x00"
    );
}

#[test]
fn signature() {
    let b = init_test!(b"", Value::Signature(String::from("i")), true);
    end_test!(b, b"\x01\x69\x00");
}

#[test]
fn signature_error() {
    let s = "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
             iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
             iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
             i"
    .to_string();
    let r = init_error_test!(b"", Value::Signature(s), true);
    if let Err(EncodeError::SignatureTooLarge(_)) = r {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd_1() {
    let mut b = BytesMut::from(&b""[..]);
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut b, &mut fds);
    let v = Value::UnixFD(1);
    encoder.value(&v, true).unwrap();
    end_test!(b, b"\x00\x00\x00\x00");
    assert_eq!(&fds[..], &[1,][..]);
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd_2() {
    let mut b = BytesMut::from(&b""[..]);
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut b, &mut fds);
    let v = Value::UnixFD(1);
    encoder.value(&v, true).unwrap();
    encoder.value(&v, true).unwrap();
    end_test!(b, b"\x00\x00\x00\x00\x00\x00\x00\x00");
    assert_eq!(&fds[..], &[1,][..]);
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd_3() {
    let mut b = BytesMut::from(&b""[..]);
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut b, &mut fds);
    let v = Value::UnixFD(1);
    encoder.value(&v, true).unwrap();
    let v = Value::UnixFD(2);
    encoder.value(&v, true).unwrap();
    end_test!(b, b"\x00\x00\x00\x00\x01\x00\x00\x00");
    assert_eq!(&fds[..], &[1, 2][..]);
}
