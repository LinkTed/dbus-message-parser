use dbus_message_parser::message::{MessageHeaderFields, MessageHeaderFieldsError};
use dbus_message_parser::value::{
    BusError, ErrorError, InterfaceError, MemberError, Struct, Type, Value, WellKnownBusNameError,
};
use std::convert::{TryFrom, TryInto};

#[test]
fn error_1() {
    let values = vec![Value::String("".to_string())];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Struct(Value::String(
            "".to_string()
        )))
    );
}

#[test]
fn error_2() {
    let struct_ = Struct::try_from(vec![Value::String("".to_string())]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Length(1))
    );
}

#[test]
fn error_3() {
    let struct_ = Struct::try_from(vec![Value::Byte(1), Value::String("".to_string())]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Variant(Value::String(
            "".to_string()
        )))
    );
}

#[test]
fn error_4() {
    let variant = Value::Variant(Box::new(Value::String("".to_string())));
    let struct_ = Struct::try_from(vec![Value::Int32(1), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Byte(Value::Int32(1)))
    );
}

#[cfg(target_family = "unix")]
#[test]
fn error_5() {
    let variant = Value::Variant(Box::new(Value::String("".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(9), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::UnixFDs(Value::String(
            "".to_string()
        )))
    );
}

#[test]
fn error_6() {
    let variant = Value::Variant(Box::new(Value::Uint32(10)));
    let struct_ = Struct::try_from(vec![Value::Byte(10), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::InvalidNumber(10))
    );
}

#[test]
fn path() {
    let variant = Value::Variant(Box::new(Value::ObjectPath(
        "/object/path".try_into().unwrap(),
    )));
    let struct_ = Struct::try_from(vec![Value::Byte(1), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().path,
        Some("/object/path".try_into().unwrap())
    );
}

#[test]
fn path_error_1() {
    let variant = Value::Variant(Box::new(Value::String("/object/path".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(1), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Path(Value::String(
            "/object/path".to_string()
        )))
    );
}

#[test]
fn path_error_2() {
    let path = Value::ObjectPath("/object/path".try_into().unwrap());
    let variant = Value::Variant(Box::new(path.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(1), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultiplePath(path))
    );
}

#[test]
fn interface() {
    let variant = Value::Variant(Box::new(Value::String("org.example.interface".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(2), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().interface,
        Some("org.example.interface".try_into().unwrap())
    );
}

#[test]
fn interface_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(2), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Interface(Value::Int32(1)))
    );
}

#[test]
fn interface_error_2() {
    let variant = Value::Variant(Box::new(Value::String(
        "/org.example.interface".to_string(),
    )));
    let struct_ = Struct::try_from(vec![Value::Byte(2), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::InterfaceError(
            InterfaceError::InvalidChar(b'/')
        ))
    );
}

#[test]
fn interface_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let struct_ = Struct::try_from(vec![Value::Byte(2), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::InterfaceError(
            InterfaceError::Empty
        ))
    );
}

#[test]
fn interface_error_4() {
    let interface = Value::String("org.example.interface".to_string());
    let variant = Value::Variant(Box::new(interface.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(2), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleInterface(interface))
    );
}

#[test]
fn member() {
    let variant = Value::Variant(Box::new(Value::String("Get".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(3), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().member,
        Some("Get".try_into().unwrap())
    );
}

#[test]
fn member_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(3), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Member(Value::Int32(1)))
    );
}

#[test]
fn member_error_2() {
    let variant = Value::Variant(Box::new(Value::String("/Get".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(3), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MemberError(
            MemberError::InvalidChar(b'/')
        ))
    );
}

#[test]
fn member_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let struct_ = Struct::try_from(vec![Value::Byte(3), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MemberError(MemberError::Empty))
    );
}

#[test]
fn member_error_4() {
    let member = Value::String("Get".to_string());
    let variant = Value::Variant(Box::new(member.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(3), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleMember(member))
    );
}

#[test]
fn error_name() {
    let variant = Value::Variant(Box::new(Value::String("error.name".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(4), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().error_name,
        Some("error.name".try_into().unwrap())
    );
}

#[test]
fn error_name_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(4), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::ErrorName(Value::Int32(1)))
    );
}

#[test]
fn error_name_error_2() {
    let variant = Value::Variant(Box::new(Value::String("/error.name".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(4), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::ErrorError(
            ErrorError::InvalidChar(b'/')
        ))
    );
}

#[test]
fn error_name_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let struct_ = Struct::try_from(vec![Value::Byte(4), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::ErrorError(ErrorError::Empty))
    );
}

#[test]
fn error_name_error_4() {
    let error_name = Value::String("error.name".to_string());
    let variant = Value::Variant(Box::new(error_name.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(4), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleErrorName(error_name))
    );
}

#[test]
fn reply_serial() {
    let variant = Value::Variant(Box::new(Value::Uint32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(5), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().reply_serial,
        Some(1)
    );
}

#[test]
fn reply_serial_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(5), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::ReplySerial(Value::Int32(1)))
    );
}

#[test]
fn reply_serial_error_2() {
    let reply_serial = Value::Uint32(1);
    let variant = Value::Variant(Box::new(reply_serial.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(5), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleReplySerial(reply_serial))
    );
}

#[test]
fn destination_1() {
    let variant = Value::Variant(Box::new(Value::String(
        "org.example.destination".to_string(),
    )));
    let struct_ = Struct::try_from(vec![Value::Byte(6), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().destination,
        Some("org.example.destination".try_into().unwrap())
    );
}

#[test]
fn destination_2() {
    let variant = Value::Variant(Box::new(Value::String(":1.10".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(6), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().destination,
        Some(":1.10".try_into().unwrap())
    );
}

#[test]
fn destination_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(6), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Destination(Value::Int32(1)))
    );
}

#[test]
fn destination_error_2() {
    let variant = Value::Variant(Box::new(Value::String(
        "/org.example.destination".to_string(),
    )));
    let struct_ = Struct::try_from(vec![Value::Byte(6), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::BusError(
            BusError::WellKnownBusNameError(WellKnownBusNameError::InvalidChar(b'/'))
        ))
    );
}

#[test]
fn destination_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let struct_ = Struct::try_from(vec![Value::Byte(6), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::BusError(
            BusError::WellKnownBusNameError(WellKnownBusNameError::Empty)
        ))
    );
}

#[test]
fn destination_error_4() {
    let destination = Value::String("org.example.destination".to_string());
    let variant = Value::Variant(Box::new(destination.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(6), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleDestination(destination))
    );
}

#[test]
fn sender_1() {
    let variant = Value::Variant(Box::new(Value::String("org.example.sender".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(7), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().sender,
        Some("org.example.sender".try_into().unwrap())
    );
}

#[test]
fn sender_2() {
    let variant = Value::Variant(Box::new(Value::String(":1.10".to_string())));
    let struct_ = Struct::try_from(vec![Value::Byte(7), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().sender,
        Some(":1.10".try_into().unwrap())
    );
}

#[test]
fn sender_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(7), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Sender(Value::Int32(1)))
    );
}

#[test]
fn sender_error_2() {
    let variant = Value::Variant(Box::new(Value::String(
        "/org.example.sender".try_into().unwrap(),
    )));
    let struct_ = Struct::try_from(vec![Value::Byte(7), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::BusError(
            BusError::WellKnownBusNameError(WellKnownBusNameError::InvalidChar(b'/'))
        ))
    );
}

#[test]
fn sender_error_3() {
    let variant = Value::Variant(Box::new(Value::String(String::new())));
    let struct_ = Struct::try_from(vec![Value::Byte(7), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::BusError(
            BusError::WellKnownBusNameError(WellKnownBusNameError::Empty)
        ))
    );
}

#[test]
fn sender_error_4() {
    let sender = Value::String("org.example.sender".to_string());
    let variant = Value::Variant(Box::new(sender.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(7), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleSender(sender))
    );
}

#[test]
fn signature() {
    let variant = Value::Variant(Box::new(Value::Signature(vec![Type::Int32])));
    let struct_ = Struct::try_from(vec![Value::Byte(8), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values).unwrap().signature,
        Some(vec![Type::Int32])
    );
}

#[test]
fn signature_error_1() {
    let variant = Value::Variant(Box::new(Value::Int32(1)));
    let struct_ = Struct::try_from(vec![Value::Byte(8), variant]).unwrap();
    let values = vec![Value::Struct(struct_)];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::Signature(Value::Int32(1)))
    );
}

#[test]
fn signature_error_2() {
    let signature = Value::Signature(vec![Type::Int32]);
    let variant = Value::Variant(Box::new(signature.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(8), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleSignature(signature))
    );
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fds_error() {
    let unix_fds = Value::Uint32(10);
    let variant = Value::Variant(Box::new(unix_fds.clone()));
    let struct_ = Struct::try_from(vec![Value::Byte(9), variant]).unwrap();
    let value = Value::Struct(struct_);
    let values = vec![value.clone(), value];
    assert_eq!(
        MessageHeaderFields::try_from(values),
        Err(MessageHeaderFieldsError::MultipleUnixFDs(unix_fds))
    );
}
