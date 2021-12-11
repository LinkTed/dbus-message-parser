use dbus_message_parser::{
    match_rule::{Arg, ArgPath, MatchRule},
    message::{Message, MessageFlags, MessageHeader, MessageHeaderFields, MessageType},
    value::{Bus, Interface, Member, ObjectPath, UniqueConnectionName, Value},
};
use std::convert::TryFrom;

fn create_message(
    message_header_fields: MessageHeaderFields,
    message_type: MessageType,
    body: Vec<Value>,
) -> Message {
    let message_header = MessageHeader::new(
        true,
        message_type,
        MessageFlags::empty(),
        1,
        1,
        message_header_fields,
    )
    .unwrap();
    Message::new(message_header, body)
}

#[test]
fn matching_rule_type() {
    let match_rule = MatchRule::Type(MessageType::Signal);

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        interface: Some(Interface::try_from("a.a").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::Signal, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_sender_1() {
    let match_rule = MatchRule::Sender(Bus::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        sender: Some(Bus::try_from("a.a").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_sender_2() {
    let match_rule = MatchRule::Sender(Bus::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_interface_1() {
    let match_rule = MatchRule::Interface(Interface::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        interface: Some(Interface::try_from("a.a").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_interface_2() {
    let match_rule = MatchRule::Interface(Interface::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_member_1() {
    let match_rule = MatchRule::Member(Member::try_from("A").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_member_2() {
    let match_rule = MatchRule::Member(Member::try_from("A").unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_1() {
    let match_rule = MatchRule::Path(ObjectPath::try_from("/").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_2() {
    let match_rule = MatchRule::Path(ObjectPath::try_from("/").unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(1),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_namespace_1() {
    let match_rule = MatchRule::PathNamespace(ObjectPath::try_from("/a").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/a/a").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_namespace_2() {
    let match_rule = MatchRule::PathNamespace(ObjectPath::try_from("/").unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(1),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_path_namespace_3() {
    let match_rule = MatchRule::PathNamespace(ObjectPath::try_from("/b").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/a/a").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_destination_1() {
    let match_rule = MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        destination: Some(Bus::try_from(":1.1").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_destination_2() {
    let match_rule = MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_destination_3() {
    let match_rule = MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap());

    let message_header_fields = MessageHeaderFields {
        path: Some(ObjectPath::try_from("/").unwrap()),
        member: Some(Member::try_from("A").unwrap()),
        destination: Some(Bus::try_from("a.a").unwrap()),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodCall, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_1() {
    let match_rule = MatchRule::Arg(Arg::try_from((0, "A".to_string())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(
        message_header_fields,
        MessageType::MethodReturn,
        vec![Value::String("A".to_string())],
    );

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_2() {
    let match_rule = MatchRule::Arg(Arg::try_from((0, "A".to_string())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_3() {
    let match_rule = MatchRule::Arg(Arg::try_from((0, "A".to_string())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_1() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(
        message_header_fields,
        MessageType::MethodReturn,
        vec![Value::String("/a/a".to_string())],
    );

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_2() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(
        message_header_fields,
        MessageType::MethodReturn,
        vec![Value::ObjectPath(ObjectPath::try_from("/a/a").unwrap())],
    );

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_3() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg_path_4() {
    let match_rule =
        MatchRule::ArgPath(ArgPath::try_from((0, ObjectPath::try_from("/a").unwrap())).unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(
        message_header_fields,
        MessageType::MethodReturn,
        vec![Value::Uint32(0)],
    );

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg0_namespace_1() {
    let match_rule = MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(
        message_header_fields,
        MessageType::MethodReturn,
        vec![Value::String("a.a.a".to_string())],
    );

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg0_namespace_2() {
    let match_rule = MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(
        message_header_fields,
        MessageType::MethodReturn,
        vec![Value::Uint32(0)],
    );

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_arg0_namespace_3() {
    let match_rule = MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap());

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(!MatchRule::matching_rules(&[match_rule], &message));
}

#[test]
fn matching_rule_eavesdrop() {
    let match_rule = MatchRule::Eavesdrop(true);

    let message_header_fields = MessageHeaderFields {
        reply_serial: Some(2),
        ..Default::default()
    };
    let message = create_message(message_header_fields, MessageType::MethodReturn, Vec::new());

    assert!(MatchRule::matching_rules(&[match_rule], &message));
}
