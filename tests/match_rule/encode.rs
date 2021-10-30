use dbus_message_parser::{
    match_rule::{Arg, ArgPath, MatchRule},
    message::MessageType,
    value::{Bus, Interface, Member, ObjectPath, UniqueConnectionName},
};
use std::convert::TryFrom;

#[test]
fn type_1() {
    assert_eq!(
        MatchRule::Type(MessageType::MethodCall).to_string(),
        "type=method_call",
    );
}

#[test]
fn type_2() {
    assert_eq!(
        MatchRule::Type(MessageType::MethodReturn).to_string(),
        "type=method_return",
    );
}

#[test]
fn type_3() {
    assert_eq!(
        MatchRule::Type(MessageType::Error).to_string(),
        "type=error",
    );
}

#[test]
fn type_4() {
    assert_eq!(
        MatchRule::Type(MessageType::Signal).to_string(),
        "type=signal",
    );
}

#[test]
fn sender() {
    assert_eq!(
        MatchRule::Sender(Bus::try_from("a.a").unwrap()).to_string(),
        "sender=a.a",
    );
}

#[test]
fn interface() {
    assert_eq!(
        MatchRule::Interface(Interface::try_from("a.a").unwrap()).to_string(),
        "interface=a.a",
    );
}

#[test]
fn member() {
    assert_eq!(
        MatchRule::Member(Member::try_from("A").unwrap()).to_string(),
        "member=A",
    );
}

#[test]
fn path() {
    assert_eq!(
        MatchRule::Path(ObjectPath::try_from("/").unwrap()).to_string(),
        "path=/",
    );
}

#[test]
fn path_namespace() {
    assert_eq!(
        MatchRule::PathNamespace(ObjectPath::try_from("/").unwrap()).to_string(),
        "path_namespace=/",
    );
}

#[test]
fn destination() {
    assert_eq!(
        MatchRule::Destination(UniqueConnectionName::try_from(":1.1").unwrap()).to_string(),
        "destination=:1.1",
    );
}

#[test]
fn arg() {
    assert_eq!(
        MatchRule::Arg(Arg::try_from((10, "A".to_string())).unwrap()).to_string(),
        "arg10=A",
    );
}

#[test]
fn arg_path() {
    assert_eq!(
        MatchRule::ArgPath(ArgPath::from((101, ObjectPath::try_from("/").unwrap()))).to_string(),
        "arg101path=/",
    );
}

#[test]
fn arg0_namespace() {
    assert_eq!(
        MatchRule::Arg0Namespace(Interface::try_from("a.a").unwrap()).to_string(),
        "arg0namespace=a.a",
    );
}

#[test]
fn eavesdrop_1() {
    assert_eq!(MatchRule::Eavesdrop(true).to_string(), "eavesdrop=true",);
}

#[test]
fn eavesdrop_2() {
    assert_eq!(MatchRule::Eavesdrop(false).to_string(), "eavesdrop=false",);
}

#[test]
fn encode() {
    assert_eq!(
        MatchRule::encode(
            &[
                MatchRule::Eavesdrop(true),
                MatchRule::Sender(Bus::try_from(":1.1").unwrap()),
                MatchRule::Interface(Interface::try_from("a.a").unwrap())
            ][..]
        ),
        "eavesdrop=true,sender=:1.1,interface=a.a",
    );
}

#[test]
fn encode_empty() {
    assert_eq!(MatchRule::encode(&[][..]), "",);
}
