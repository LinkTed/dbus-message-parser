use dbus_message_parser::{
    match_rule::{Arg, ArgPath, MatchRule},
    message::{Message, MessageFlags, MessageHeader, MessageHeaderField, MessageType},
    value::{Bus, Interface, Member, ObjectPath, UniqueConnectionName, Value},
};
use std::{collections::BTreeSet, convert::TryFrom};

#[test]
fn matching_rule_type() {
    let match_rule = MatchRule::Type(MessageType::Signal);

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Interface(
        Interface::try_from("a.a").unwrap(),
    ));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::Signal,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_sender_1() {
    let match_rule = MatchRule::Sender(Bus::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    message_header_field.insert(MessageHeaderField::Sender(Bus::try_from("a.a").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_sender_2() {
    let match_rule = MatchRule::Sender(Bus::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_interface_1() {
    let match_rule = MatchRule::Interface(Interface::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    message_header_field.insert(MessageHeaderField::Interface(
        Interface::try_from("a.a").unwrap(),
    ));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_interface_2() {
    let match_rule = MatchRule::Interface(Interface::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_member_1() {
    let match_rule = MatchRule::Member(Member::try_from("A").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_member_2() {
    let match_rule = MatchRule::Member(Member::try_from("A").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_1() {
    let match_rule = MatchRule::Path(ObjectPath::try_from("/").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_2() {
    let match_rule = MatchRule::Path(ObjectPath::try_from("/").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(1));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_namespace_1() {
    let match_rule = MatchRule::PathNamespace(ObjectPath::try_from("/a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(
        ObjectPath::try_from("/a/a").unwrap(),
    ));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_namespace_2() {
    let match_rule = MatchRule::PathNamespace(ObjectPath::try_from("/").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(1));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_namespace_3() {
    let match_rule = MatchRule::PathNamespace(ObjectPath::try_from("/b").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(
        ObjectPath::try_from("/a/a").unwrap(),
    ));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_destination_1() {
    let match_rule = MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    message_header_field.insert(MessageHeaderField::Destination(
        Bus::try_from(":1.1").unwrap(),
    ));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_destination_2() {
    let match_rule = MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_destination_3() {
    let match_rule = MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::Path(ObjectPath::try_from("/").unwrap()));
    message_header_field.insert(MessageHeaderField::Member(Member::try_from("A").unwrap()));
    message_header_field.insert(MessageHeaderField::Destination(
        Bus::try_from("a.a").unwrap(),
    ));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_1() {
    let match_rule = MatchRule::Arg(Arg::try_from((0, "A".to_string())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, vec![Value::String("A".to_string())]);

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_2() {
    let match_rule = MatchRule::Arg(Arg::try_from((0, "A".to_string())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, vec![Value::Uint32(0)]);

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_3() {
    let match_rule = MatchRule::Arg(Arg::try_from((0, "A".to_string())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_1() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, vec![Value::String("/a/a".to_string())]);

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_2() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(
        message_header,
        vec![Value::ObjectPath(ObjectPath::try_from("/a/a").unwrap())],
    );

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_3() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_4() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, vec![Value::Uint32(0)]);

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg0_namespace_1() {
    let match_rule = MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, vec![Value::String("a.a.a".to_string())]);

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg0_namespace_2() {
    let match_rule = MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, vec![Value::Uint32(0)]);

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg0_namespace_3() {
    let match_rule = MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap());

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_eavesdrop() {
    let match_rule = MatchRule::Eavesdrop(true);

    let mut message_header_field = BTreeSet::new();
    message_header_field.insert(MessageHeaderField::ReplySerial(2));
    let message_header = MessageHeader::new(
        true,
        MessageType::MethodReturn,
        MessageFlags::empty(),
        1,
        1,
        message_header_field,
    )
    .unwrap();
    let message = Message::new(message_header, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}
