use dbus_message_parser::message::{
    MessageFlags, MessageHeader, MessageHeaderError, MessageHeaderFields, MessageType,
};
use std::convert::TryInto;

fn create_header(
    message_type: MessageType,
    fields: MessageHeaderFields,
) -> Result<MessageHeader, MessageHeaderError> {
    MessageHeader::new(true, message_type, MessageFlags::empty(), 1, 0, fields)
}

#[test]
fn method_call() {
    let message_type = MessageType::MethodCall;

    let fields = MessageHeaderFields {
        path: Some("/object/path".try_into().unwrap()),
        member: Some("MethodName".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn method_call_error_1() {
    let message_type = MessageType::MethodCall;

    let fields = MessageHeaderFields {
        member: Some("MethodName".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingPath))
}

#[test]
fn method_call_error_2() {
    let message_type = MessageType::MethodCall;

    let fields = MessageHeaderFields {
        path: Some("/object/path".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingMember))
}

#[test]
fn signal() {
    let message_type = MessageType::Signal;

    let fields = MessageHeaderFields {
        path: Some("/object/path".try_into().unwrap()),
        interface: Some("interface.name".try_into().unwrap()),
        member: Some("MethodName".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn signal_error_1() {
    let message_type = MessageType::Signal;

    let fields = MessageHeaderFields {
        interface: Some("interface.name".try_into().unwrap()),
        member: Some("MethodName".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingPath))
}

#[test]
fn signal_error_2() {
    let message_type = MessageType::Signal;

    let fields = MessageHeaderFields {
        path: Some("/object/path".try_into().unwrap()),
        member: Some("MethodName".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingInterface))
}

#[test]
fn signal_error_3() {
    let message_type = MessageType::Signal;

    let fields = MessageHeaderFields {
        path: Some("/object/path".try_into().unwrap()),
        interface: Some("interface.name".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingMember));
}

#[test]
fn error() {
    let message_type = MessageType::Error;

    let fields = MessageHeaderFields {
        error_name: Some("error.name".try_into().unwrap()),
        reply_serial: Some(1),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn error_error_1() {
    let message_type = MessageType::Error;

    let fields = MessageHeaderFields {
        reply_serial: Some(1),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingErrorName))
}

#[test]
fn error_error_2() {
    let message_type = MessageType::Error;

    let fields = MessageHeaderFields {
        error_name: Some("error.name".try_into().unwrap()),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingReplySerial))
}

#[test]
fn method_return() {
    let message_type = MessageType::MethodReturn;

    let fields = MessageHeaderFields {
        reply_serial: Some(1),
        ..Default::default()
    };

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn method_return_error() {
    let message_type = MessageType::MethodReturn;

    let fields = MessageHeaderFields::default();

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingReplySerial))
}
