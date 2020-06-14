use bytes::Bytes;
use dbus_message_parser::{DecodeError, Decoder, Value};

macro_rules! init_test {
    ($array:tt, $le:expr, $sig:expr) => {{
        let b = Bytes::from_static(&$array[..]);
        let mut decoder = Decoder::new(&b);
        let mut v = decoder.value($le, $sig).unwrap();
        assert_eq!(v.len(), 1);
        v.pop().unwrap()
    }};
    ($array:tt, $offset:expr, $le:expr, $sig:expr) => {{
        let b = Bytes::from_static(&$array[..]);
        let mut decoder = Decoder::with_offset(&b, $offset);
        let mut v = decoder.value($le, $sig).unwrap();
        assert_eq!(v.len(), 1);
        v.pop().unwrap()
    }};
}

macro_rules! init_error_test {
    ($array:tt, $sig:expr) => {{
        let b = Bytes::from_static(&$array[..]);
        let mut decoder = Decoder::new(&b);
        decoder.value(true, $sig)
    }};
}

#[test]
fn byte() {
    let v = init_test!(b"\x01", true, "y");
    assert_eq!(v, Value::Byte(1));
}

#[test]
fn byte_error() {
    let v = init_error_test!(b"", "y");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn boolean_1() {
    let v = init_test!(b"\x00\x00\x00\x00", true, "b");
    assert_eq!(v, Value::Boolean(false));
}

#[test]
fn boolean_2() {
    let v = init_test!(b"\x01\x00\x00\x00", true, "b");
    assert_eq!(v, Value::Boolean(true));
}

#[test]
fn boolean_3() {
    let v = init_test!(b"\x00\x00\x00\x00\x01\x00\x00\x00", 1, true, "b");
    assert_eq!(v, Value::Boolean(true));
}

#[test]
fn boolean_4() {
    let v = init_test!(b"\x00\x00\x00\x01", false, "b");
    assert_eq!(v, Value::Boolean(true));
}

#[test]
fn boolean_error_1() {
    let v = init_error_test!(b"\x00\x00\x00", "b");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn boolean_error_2() {
    let v = init_error_test!(b"\x03\x00\x00\x00", "b");
    assert_eq!(v, Err(DecodeError::InvalidBoolean(3)));
}

#[test]
fn int_16_1() {
    let v = init_test!(b"\x01\xf0", true, "n");
    assert_eq!(v, Value::Int16(-4095));
}

#[test]
fn int_16_2() {
    let v = init_test!(b"\xf0\x01", false, "n");
    assert_eq!(v, Value::Int16(-4095));
}

#[test]
fn int_16_3() {
    let v = init_test!(b"\x00\x00\x01\xf0", 1, true, "n");
    assert_eq!(v, Value::Int16(-4095));
}

#[test]
fn int_16_error() {
    let v = init_error_test!(b"\x0f", "n");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn uint_16_1() {
    let v = init_test!(b"\x01\xf0", true, "q");
    assert_eq!(v, Value::Uint16(61441));
}

#[test]
fn uint_16_2() {
    let v = init_test!(b"\xf0\x01", false, "q");
    assert_eq!(v, Value::Uint16(61441));
}

#[test]
fn uint_16_3() {
    let v = init_test!(b"\x00\x00\x01\xf0", 1, true, "q");
    assert_eq!(v, Value::Uint16(61441));
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd() {
    let b = Bytes::from_static(&b"\x00\x00\x00\x00"[..]);
    let fds = [2];
    let mut decoder = Decoder::with_fds(&b, 0, &fds[..], 0);
    let mut v = decoder.value(true, "h").unwrap();
    assert_eq!(v.len(), 1);
    let v = v.pop().unwrap();
    assert_eq!(v, Value::UnixFD(2));
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd_error() {
    let b = Bytes::from_static(&b"\x01\x00\x00\x00"[..]);
    let fds = [2];
    let mut decoder = Decoder::with_fds(&b, 0, &fds[..], 0);
    let v = decoder.value(true, "h");
    assert_eq!(v, Err(DecodeError::OutOfBoundsFds));
}

#[test]
fn uint_16_error() {
    let v = init_error_test!(b"\x01", "q");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn int_32_1() {
    let v = init_test!(b"\x01\x01\x01\xf0", true, "i");
    assert_eq!(v, Value::Int32(-268369663));
}

#[test]
fn int_32_2() {
    let v = init_test!(b"\xf0\x01\x01\x01", false, "i");
    assert_eq!(v, Value::Int32(-268369663));
}

#[test]
fn int_32_3() {
    let v = init_test!(b"\x00\x00\x00\x00\x01\x01\x01\xf0", 1, true, "i");
    assert_eq!(v, Value::Int32(-268369663));
}

#[test]
fn int_32_error() {
    let v = init_error_test!(b"\x01\x01\x01", "i");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn uint_32_1() {
    let v = init_test!(b"\x01\x01\x01\xf0", true, "u");
    assert_eq!(v, Value::Uint32(4026597633));
}

#[test]
fn uint_32_2() {
    let v = init_test!(b"\xf0\x01\x01\x01", false, "u");
    assert_eq!(v, Value::Uint32(4026597633));
}

#[test]
fn uint_32_3() {
    let v = init_test!(b"\x00\x00\x00\x00\x01\x01\x01\xf0", 1, true, "u");
    assert_eq!(v, Value::Uint32(4026597633));
}

#[test]
fn uint_32_error() {
    let v = init_error_test!(b"\x01\x01\x01", "u");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn int_64_1() {
    let v = init_test!(b"\x01\x01\x01\x01\x01\x01\x01\xf0", true, "x");
    assert_eq!(v, Value::Int64(-1152638925806698239));
}

#[test]
fn int_64_2() {
    let v = init_test!(b"\xf0\x01\x01\x01\x01\x01\x01\x01", false, "x");
    assert_eq!(v, Value::Int64(-1152638925806698239));
}

#[test]
fn int_64_3() {
    let v = init_test!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x01\xf0",
        1,
        true,
        "x"
    );
    assert_eq!(v, Value::Int64(-1152638925806698239));
}

#[test]
fn int_64_error() {
    let v = init_error_test!(b"\x01\x01\x01\x01\x01\x01\x01", "x");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn uint_64_1() {
    let v = init_test!(b"\x01\x01\x01\x01\x01\x01\x01\xf0", true, "t");
    assert_eq!(v, Value::Uint64(17294105147902853377));
}

#[test]
fn uint_64_2() {
    let v = init_test!(b"\xf0\x01\x01\x01\x01\x01\x01\x01", false, "t");
    assert_eq!(v, Value::Uint64(17294105147902853377));
}

#[test]
fn uint_64_3() {
    let v = init_test!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x01\xf0",
        1,
        true,
        "t"
    );
    assert_eq!(v, Value::Uint64(17294105147902853377));
}

#[test]
fn uint_64_error() {
    let v = init_error_test!(b"\x01\x01\x01\x01\x01\x01\x01", "t");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn double_1() {
    let v = init_test!(b"\x77\xbe\x9f\x1a\x2f\xdd\x5e\xc0", true, "d");
    assert_eq!(v, Value::Double(-123.456));
}

#[test]
fn double_2() {
    let v = init_test!(b"\xc0\x5e\xdd\x2f\x1a\x9f\xbe\x77", false, "d");
    assert_eq!(v, Value::Double(-123.456));
}

#[test]
fn double_3() {
    let v = init_test!(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x77\xbe\x9f\x1a\x2f\xdd\x5e\xc0",
        1,
        true,
        "d"
    );
    assert_eq!(v, Value::Double(-123.456));
}

#[test]
fn double_error() {
    let v = init_error_test!(b"\x77\xbe\x9f\x1a\x2f\xdd\x5e", "d");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn string_1() {
    let v = init_test!(b"\x03\x00\x00\x00\x66\x6f\x6f\x00", true, "s");
    assert_eq!(v, Value::String(String::from("foo")));
}

#[test]
fn string_2() {
    let v = init_test!(b"\x00\x00\x00\x03\x66\x6f\x6f\x00", false, "s");
    assert_eq!(v, Value::String(String::from("foo")));
}

#[test]
fn string_3() {
    let v = init_test!(
        b"\x00\x00\x00\x00\x03\x00\x00\x00\x66\x6f\x6f\x00",
        1,
        true,
        "s"
    );
    assert_eq!(v, Value::String(String::from("foo")));
}

#[test]
fn string_error_1() {
    let v = init_error_test!(b"\x00\x00\x00", "s");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn string_error_2() {
    let v = init_error_test!(b"\x03\x00\x00\x00\x66\x6f\x6f\x30", "s");
    assert_eq!(v, Err(DecodeError::StringNotNull));
}

#[test]
fn string_error_3() {
    let v = init_error_test!(b"\x03\x00\x00\x00\x66\x6f\x6f", "s");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn path_1() {
    let v = init_test!(b"\x05\x00\x00\x00\x2f\x74\x65\x73\x74\x00", true, "o");
    assert_eq!(v, Value::ObjectPath(String::from("/test")));
}

#[test]
fn path_2() {
    let v = init_test!(b"\x00\x00\x00\x05\x2f\x74\x65\x73\x74\x00", false, "o");
    assert_eq!(v, Value::ObjectPath(String::from("/test")));
}

#[test]
fn path_3() {
    let v = init_test!(
        b"\x00\x00\x00\x00\x05\x00\x00\x00\x2f\x74\x65\x73\x74\x00",
        1,
        true,
        "o"
    );
    assert_eq!(v, Value::ObjectPath(String::from("/test")));
}

#[test]
fn path_error() {
    let v = init_error_test!(b"\x03\x00\x00\x00\x66\x6f\x6f\x00", "o");
    assert_eq!(v, Err(DecodeError::ObjectPathRegex));
}

#[test]
fn signature() {
    let v = init_test!(b"\x01\x69\x00", true, "g");
    assert_eq!(v, Value::Signature(String::from("i")));
}

#[test]
fn signature_error_1() {
    let v = init_error_test!(b"", "g");
    assert_eq!(v, Err(DecodeError::TooShort));
}

#[test]
fn signature_error_2() {
    let v = init_error_test!(b"\x01\x69\x30", "g");
    assert_eq!(v, Err(DecodeError::StringNotNull));
}

#[test]
fn signature_error_3() {
    let v = init_error_test!(b"\x01\x69", "g");
    assert_eq!(v, Err(DecodeError::TooShort));
}
