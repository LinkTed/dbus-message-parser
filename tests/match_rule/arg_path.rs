use dbus_message_parser::{match_rule::ArgPath, value::ObjectPath};
use std::convert::TryFrom;

#[test]
fn get_value() {
    let arg_path = ArgPath::from((0, ObjectPath::try_from("/").unwrap()));
    assert_eq!("/", arg_path.get_value().as_ref());
}
