use dbus_message_parser::message::{MessageHeaderField, MessageHeaderFieldError};
use dbus_message_parser::value::{BusError, ErrorError, InterfaceError, MemberError, Type, Value};
use std::convert::{TryFrom, TryInto};

#[test]
fn error_1() {
    let value = Value::String("".to_string());
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Struct(Value::String(
            "".to_string()
        )))
    );
}

#[test]
fn error_2() {
    let value = Value::Struct(vec![Value::String("".to_string())]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Length(1))
    );
}

#[test]
fn error_3() {
    let value = Value::Struct(vec![Value::Byte(1), Value::String("".to_string())]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Variant(Value::String(
            "".to_string()
        )))
    );
}

#[test]
fn error_4() {
    let variant = Value::Variant(Box::new(Value::String("".to_string())));
    let value = Value::Struct(vec![Value::Int32(1), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Byte(Value::Int32(1)))
    );
}

#[cfg(target_family = "unix")]
#[test]
fn error_5() {
    let variant = Value::Variant(Box::new(Value::String("".to_string())));
    let value = Value::Struct(vec![Value::Byte(9), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::UnixFDs(Value::String(
            "".to_string()
        )))
    );
}

#[test]
fn path() {
    let variant = Value::Variant(Box::new(Value::ObjectPath(
        "/object/path".try_into().unwrap(),
    )));
    let value = Value::Struct(vec![Value::Byte(1), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Path("/object/path".try_into().unwrap()))
    );
}

#[test]
fn path_error() {
    let variant = Value::Variant(Box::new(Value::String("/object/path".to_string())));
    let value = Value::Struct(vec![Value::Byte(1), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Path(Value::String(
            "/object/path".to_string()
        )))
    );
}

#[test]
fn interface() {
    let variant = Value::Variant(Box::new(Value::String("org.example.interface".to_string())));
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Interface(
            "org.example.interface".try_into().unwrap()
        ))
    );
}

#[test]
fn interface_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Interface(Value::Int32(1)))
    );
}

#[test]
fn interface_error_2() {
    let variant = Value::Variant(Box::new(Value::String(
        "/org.example.interface".to_string(),
    )));
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::InterfaceError(
            InterfaceError::InvalidChar(b'/')
        ))
    );
}

#[test]
fn interface_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let value = Value::Struct(vec![Value::Byte(2), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::InterfaceError(
            InterfaceError::Empty
        ))
    );
}

#[test]
fn member() {
    let variant = Value::Variant(Box::new(Value::String("Get".to_string())));
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Member("Get".try_into().unwrap()))
    );
}

#[test]
fn member_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Member(Value::Int32(1)))
    );
}

#[test]
fn member_error_2() {
    let variant = Value::Variant(Box::new(Value::String("/Get".to_string())));
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::MemberError(
            MemberError::InvalidChar(b'/')
        ))
    );
}

#[test]
fn member_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let value = Value::Struct(vec![Value::Byte(3), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::MemberError(MemberError::Empty))
    );
}

#[test]
fn error_name() {
    let variant = Value::Variant(Box::new(Value::String("error.name".to_string())));
    let value = Value::Struct(vec![Value::Byte(4), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::ErrorName(
            "error.name".try_into().unwrap()
        ))
    );
}

#[test]
fn error_name_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(4), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::ErrorName(Value::Int32(1)))
    );
}

#[test]
fn error_name_error_2() {
    let variant = Value::Variant(Box::new(Value::String("/error.name".to_string())));
    let value = Value::Struct(vec![Value::Byte(4), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::ErrorError(
            ErrorError::InvalidChar(b'/')
        ))
    );
}

#[test]
fn error_name_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let value = Value::Struct(vec![Value::Byte(4), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::ErrorError(ErrorError::Empty))
    );
}

#[test]
fn reply_serial() {
    let variant = Value::Variant(Box::new(Value::Uint32(1)));
    let value = Value::Struct(vec![Value::Byte(5), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::ReplySerial(1))
    );
}

#[test]
fn reply_serial_error() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(5), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::ReplySerial(Value::Int32(1)))
    );
}

#[test]
fn destination_1() {
    let variant = Value::Variant(Box::new(Value::String(
        "org.example.destination".to_string(),
    )));
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Destination(
            "org.example.destination".try_into().unwrap()
        ))
    );
}

#[test]
fn destination_2() {
    let variant = Value::Variant(Box::new(Value::String(":1.10".to_string())));
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Destination(":1.10".try_into().unwrap()))
    );
}

#[test]
fn destination_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Destination(Value::Int32(1)))
    );
}

#[test]
fn destination_error_2() {
    let variant = Value::Variant(Box::new(Value::String(
        "/org.example.destination".to_string(),
    )));
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::BusError(BusError::InvalidChar(
            b'/'
        )))
    );
}

#[test]
fn destination_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let value = Value::Struct(vec![Value::Byte(6), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::BusError(BusError::Empty))
    );
}

#[test]
fn sender_1() {
    let variant = Value::Variant(Box::new(Value::String("org.example.sender".to_string())));
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Sender(
            "org.example.sender".try_into().unwrap()
        ))
    );
}

#[test]
fn sender_2() {
    let variant = Value::Variant(Box::new(Value::String(":1.10".to_string())));
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Sender(":1.10".try_into().unwrap()))
    );
}

#[test]
fn sender_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Sender(Value::Int32(1)))
    );
}

#[test]
fn sender_error_2() {
    let variant = Value::Variant(Box::new(Value::String(
        "/org.example.sender".try_into().unwrap(),
    )));
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::BusError(BusError::InvalidChar(
            b'/'
        )))
    );
}

#[test]
fn sender_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let value = Value::Struct(vec![Value::Byte(7), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::BusError(BusError::Empty))
    );
}

#[test]
fn signature() {
    let variant = Value::Variant(Box::new(Value::Signature(vec![Type::Int32])));
    let value = Value::Struct(vec![Value::Byte(8), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Ok(MessageHeaderField::Signature(vec![Type::Int32]))
    );
}

#[test]
fn signature_error() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let value = Value::Struct(vec![Value::Byte(8), variant]);
    assert_eq!(
        MessageHeaderField::try_from(value),
        Err(MessageHeaderFieldError::Signature(Value::Int32(1)))
    );
}
