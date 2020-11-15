use dbus_message_parser::{Header, MessageFlags, MessageHeader, MessageHeaderError, MessageType};
use std::collections::BTreeSet;
use std::convert::TryInto;

fn create_header(
    message_type: MessageType,
    header_fields: BTreeSet<Header>,
) -> Result<MessageHeader, MessageHeaderError> {
    MessageHeader::new(
        true,
        message_type,
        MessageFlags::empty(),
        1,
        0,
        header_fields,
    )
}

#[test]
fn method_call() {
    let message_type = MessageType::MethodCall;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Path("/object/path".try_into().unwrap()));
    header_fields.insert(Header::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert!(message_header.is_ok())
}

#[test]
fn method_call_error_1() {
    let message_type = MessageType::MethodCall;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingPath))
}

#[test]
fn method_call_error_2() {
    let message_type = MessageType::MethodCall;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Path("/object/path".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingMember))
}

#[test]
fn signal() {
    let message_type = MessageType::Signal;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Path("/object/path".try_into().unwrap()));
    header_fields.insert(Header::Interface("interface.name".try_into().unwrap()));
    header_fields.insert(Header::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert!(message_header.is_ok())
}

#[test]
fn signal_error_1() {
    let message_type = MessageType::Signal;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Interface("interface.name".try_into().unwrap()));
    header_fields.insert(Header::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingPath))
}

#[test]
fn signal_error_2() {
    let message_type = MessageType::Signal;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Path("/object/path".try_into().unwrap()));
    header_fields.insert(Header::Member("MethodName".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingInterface))
}

#[test]
fn signal_error_3() {
    let message_type = MessageType::Signal;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::Path("/object/path".try_into().unwrap()));
    header_fields.insert(Header::Interface("interface.name".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingMember));
}

#[test]
fn error() {
    let message_type = MessageType::Error;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::ErrorName("error.name".try_into().unwrap()));
    header_fields.insert(Header::ReplySerial(1));

    let message_header = create_header(message_type, header_fields);
    assert!(message_header.is_ok())
}

#[test]
fn error_error_1() {
    let message_type = MessageType::Error;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::ReplySerial(1));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingErrorName))
}

#[test]
fn error_error_2() {
    let message_type = MessageType::Error;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::ErrorName("error.name".try_into().unwrap()));

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingReplySerial))
}

#[test]
fn method_return() {
    let message_type = MessageType::MethodReturn;

    let mut header_fields = BTreeSet::new();
    header_fields.insert(Header::ReplySerial(1));

    let message_header = create_header(message_type, header_fields);
    assert!(message_header.is_ok())
}

#[test]
fn method_return_error() {
    let message_type = MessageType::MethodReturn;

    let header_fields = BTreeSet::new();

    let message_header = create_header(message_type, header_fields);
    assert_eq!(message_header, Err(MessageHeaderError::MissingReplySerial))
}
