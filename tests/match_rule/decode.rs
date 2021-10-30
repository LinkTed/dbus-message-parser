use dbus_message_parser::{
    match_rule::{Arg, ArgPath, MatchRule, MatchRuleError},
    message::MessageType,
    value::{
        Bus, Interface, InterfaceError, Member, ObjectPath, ObjectPathError, UniqueConnectionName,
    },
};
use std::convert::TryFrom;

#[test]
fn type_1() {
    assert_eq!(
        MatchRule::decode("type=method_call").unwrap(),
        vec![MatchRule::Type(MessageType::MethodCall)]
    );
}

#[test]
fn type_2() {
    assert_eq!(
        MatchRule::decode("type=method_return").unwrap(),
        vec![MatchRule::Type(MessageType::MethodReturn)]
    );
}

#[test]
fn type_3() {
    assert_eq!(
        MatchRule::decode("type=error").unwrap(),
        vec![MatchRule::Type(MessageType::Error)]
    );
}

#[test]
fn type_4() {
    assert_eq!(
        MatchRule::decode("type=signal").unwrap(),
        vec![MatchRule::Type(MessageType::Signal)]
    );
}

#[test]
fn type_error() {
    assert_eq!(
        MatchRule::decode("type=aaa"),
        Err(MatchRuleError::TypeUnknown)
    );
}

#[test]
fn sender() {
    assert_eq!(
        MatchRule::decode("sender=:1.1").unwrap(),
        vec![MatchRule::Sender(Bus::try_from(":1.1").unwrap())]
    );
}

#[test]
fn interface() {
    assert_eq!(
        MatchRule::decode("interface=a.a").unwrap(),
        vec![MatchRule::Interface(Interface::try_from("a.a").unwrap())]
    );
}

#[test]
fn interface_error() {
    assert_eq!(
        MatchRule::decode("interface=1.a"),
        Err(MatchRuleError::InterfaceError(
            InterfaceError::ElementBeginDigit
        ))
    );
}

#[test]
fn member() {
    assert_eq!(
        MatchRule::decode("member=A").unwrap(),
        vec![MatchRule::Member(Member::try_from("A").unwrap())]
    );
}

#[test]
fn path() {
    assert_eq!(
        MatchRule::decode("path=/").unwrap(),
        vec![MatchRule::Path(ObjectPath::try_from("/").unwrap())]
    );
}

#[test]
fn path_error() {
    assert_eq!(
        MatchRule::decode("path=a"),
        Err(MatchRuleError::PathError(
            ObjectPathError::BeginAlphanumericAndUnderscoreAndHyphen
        ))
    );
}

#[test]
fn path_namespace() {
    assert_eq!(
        MatchRule::decode("path_namespace=/").unwrap(),
        vec![MatchRule::PathNamespace(ObjectPath::try_from("/").unwrap())]
    );
}

#[test]
fn path_namespace_error() {
    assert_eq!(
        MatchRule::decode("path_namespace=a"),
        Err(MatchRuleError::PathErrorNamespace(
            ObjectPathError::BeginAlphanumericAndUnderscoreAndHyphen
        ))
    );
}

#[test]
fn destination() {
    assert_eq!(
        MatchRule::decode("destination=:1.1").unwrap(),
        vec![MatchRule::Destination(
            UniqueConnectionName::try_from(":1.1").unwrap()
        )]
    );
}

#[test]
fn arg_path() {
    assert_eq!(
        MatchRule::decode("arg1path=/").unwrap(),
        vec![MatchRule::ArgPath(ArgPath::from((
            1,
            ObjectPath::try_from("/").unwrap()
        )))]
    );
}

#[test]
fn arg_path_error() {
    assert_eq!(
        MatchRule::decode("arg1path="),
        Err(MatchRuleError::ArgPathError(ObjectPathError::Empty))
    );
}

#[test]
fn arg0_namespace() {
    assert_eq!(
        MatchRule::decode("arg0namespace=a.a").unwrap(),
        vec![MatchRule::Arg0Namespace(
            Interface::try_from("a.a").unwrap()
        )]
    );
}

#[test]
fn arg0_namespace_error() {
    assert_eq!(
        MatchRule::decode("arg0namespace=1.a"),
        Err(MatchRuleError::Arg0NamespaceError(
            InterfaceError::ElementBeginDigit
        ))
    );
}

#[test]
fn arg() {
    assert_eq!(
        MatchRule::decode("arg4=1").unwrap(),
        vec![MatchRule::Arg(Arg::try_from((4, "1".to_string())).unwrap())]
    );
}

#[test]
fn eavesdrop_1() {
    assert_eq!(
        MatchRule::decode("eavesdrop=true").unwrap(),
        vec![MatchRule::Eavesdrop(true)]
    );
}

#[test]
fn eavesdrop_2() {
    assert_eq!(
        MatchRule::decode("eavesdrop=false").unwrap(),
        vec![MatchRule::Eavesdrop(false)]
    );
}

#[test]
fn eavesdrop_error() {
    assert_eq!(
        MatchRule::decode("eavesdrop=a"),
        Err(MatchRuleError::EavesdropUnknown)
    );
}

#[test]
fn key_unknown_error() {
    assert_eq!(MatchRule::decode("a=a"), Err(MatchRuleError::KeyUnknown));
}
