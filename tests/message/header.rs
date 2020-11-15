use dbus_message_parser::{
    MessageFlags, MessageHeader, MessageHeaderError, MessageHeaderField, MessageType,
};
use std::collections::BTreeSet;
use std::convert::TryInto;

fn create_header(
    message_type: MessageType,
    fields: BTreeSet<MessageHeaderField>,
) -> Result<MessageHeader, MessageHeaderError> {
    MessageHeader::new(true, message_type, MessageFlags::empty(), 1, 0, fields)
}

#[test]
fn method_call() {
    let message_type = MessageType::MethodCall;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Path("/object/path".try_into().unwrap()));
    fields.insert(MessageHeaderField::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn method_call_error_1() {
    let message_type = MessageType::MethodCall;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingPath))
}

#[test]
fn method_call_error_2() {
    let message_type = MessageType::MethodCall;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Path("/object/path".try_into().unwrap()));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingMember))
}

#[test]
fn signal() {
    let message_type = MessageType::Signal;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Path("/object/path".try_into().unwrap()));
    fields.insert(MessageHeaderField::Interface(
        "interface.name".try_into().unwrap(),
    ));
    fields.insert(MessageHeaderField::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn signal_error_1() {
    let message_type = MessageType::Signal;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Interface(
        "interface.name".try_into().unwrap(),
    ));
    fields.insert(MessageHeaderField::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingPath))
}

#[test]
fn signal_error_2() {
    let message_type = MessageType::Signal;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Path("/object/path".try_into().unwrap()));
    fields.insert(MessageHeaderField::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingInterface))
}

#[test]
fn signal_error_3() {
    let message_type = MessageType::Signal;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Path("/object/path".try_into().unwrap()));
    fields.insert(MessageHeaderField::Interface(
        "interface.name".try_into().unwrap(),
    ));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingMember));
}

#[test]
fn error() {
    let message_type = MessageType::Error;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::ErrorName(
        "error.name".try_into().unwrap(),
    ));
    fields.insert(MessageHeaderField::ReplySerial(1));

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn error_error_1() {
    let message_type = MessageType::Error;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::ReplySerial(1));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingErrorName))
}

#[test]
fn error_error_2() {
    let message_type = MessageType::Error;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::ErrorName(
        "error.name".try_into().unwrap(),
    ));

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingReplySerial))
}

#[test]
fn method_return() {
    let message_type = MessageType::MethodReturn;

    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::ReplySerial(1));

    let message_header = create_header(message_type, fields);
    assert!(message_header.is_ok())
}

#[test]
fn method_return_error() {
    let message_type = MessageType::MethodReturn;

    let fields = BTreeSet::new();

    let message_header = create_header(message_type, fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingReplySerial))
}
