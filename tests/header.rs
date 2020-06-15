use dbus_message_parser::{DecodeError, Header, Value};
use std::convert::TryFrom;

#[test]
fn error_1() {
    let value = Value::String("".to_string());
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn error_2() {
    let value = Value::Struct(vec![Value::String("".to_string())]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn error_3() {
    let value = Value::Struct(vec![Value::Byte(1), Value::String("".to_string())]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn error_4() {
    let variant = Value::Variant(vec![]);
    let value = Value::Struct(vec![Value::Byte(1), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn error_5() {
    let variant = Value::Variant(vec![Value::String("".to_string())]);
    let value = Value::Struct(vec![Value::Int32(1), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn error_6() {
    let variant = Value::Variant(vec![Value::String("".to_string())]);
    let value = Value::Struct(vec![Value::Byte(9), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn path() {
    let variant = Value::Variant(vec![Value::ObjectPath("/object/path".to_string())]);
    let value = Value::Struct(vec![Value::Byte(1), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::Path("/object/path".to_string()))
    );
}

#[test]
fn path_error_1() {
    let variant = Value::Variant(vec![Value::String("object/path".to_string())]);
    let value = Value::Struct(vec![Value::Byte(1), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn path_error_2() {
    let variant = Value::Variant(vec![Value::ObjectPath("object/path".to_string())]);
    let value = Value::Struct(vec![Value::Byte(1), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::ObjectPathRegex));
}

#[test]
fn interface() {
    let variant = Value::Variant(vec![Value::String("org.example.interface".to_string())]);
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::Interface("org.example.interface".to_string()))
    );
}

#[test]
fn interface_error_1() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn interface_error_2() {
    let variant = Value::Variant(vec![Value::String("/org.example.interface".to_string())]);
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::InterfaceRegex));
}

#[test]
fn member() {
    let variant = Value::Variant(vec![Value::String("Get".to_string())]);
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::Member("Get".to_string()))
    );
}

#[test]
fn member_error_1() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn member_error_2() {
    let variant = Value::Variant(vec![Value::String("/Get".to_string())]);
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::MemberRegex));
}

#[test]
fn error_name() {
    let variant = Value::Variant(vec![Value::String("error.name".to_string())]);
    let value = Value::Struct(vec![Value::Byte(4), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::ErrorName("error.name".to_string()))
    );
}

#[test]
fn error_name_error() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(4), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn reply_serial() {
    let variant = Value::Variant(vec![Value::Uint32(1)]);
    let value = Value::Struct(vec![Value::Byte(5), variant]);
    assert_eq!(Header::try_from(value), Ok(Header::ReplySerial(1)));
}

#[test]
fn reply_serial_error() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(5), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn destination() {
    let variant = Value::Variant(vec![Value::String("org.example.destination".to_string())]);
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::Destination("org.example.destination".to_string()))
    );
}

#[test]
fn destination_error_1() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn destination_error_2() {
    let variant = Value::Variant(vec![Value::String("/org.example.destination".to_string())]);
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::BusNamesRegex));
}

#[test]
fn sender() {
    let variant = Value::Variant(vec![Value::String("org.example.sender".to_string())]);
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::Sender("org.example.sender".to_string()))
    );
}

#[test]
fn sender_error_1() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}

#[test]
fn sender_error_2() {
    let variant = Value::Variant(vec![Value::String("/org.example.sender".to_string())]);
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::BusNamesRegex));
}

#[test]
fn signature() {
    let variant = Value::Variant(vec![Value::Signature("i".to_string())]);
    let value = Value::Struct(vec![Value::Byte(8), variant]);
    assert_eq!(
        Header::try_from(value),
        Ok(Header::Signature("i".to_string()))
    );
}

#[test]
fn signature_error() {
    let variant = Value::Variant(vec![Value::Int32(1)]);
    let value = Value::Struct(vec![Value::Byte(8), variant]);
    assert_eq!(Header::try_from(value), Err(DecodeError::Header));
}
