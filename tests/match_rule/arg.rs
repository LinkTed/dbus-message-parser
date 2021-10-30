use dbus_message_parser::match_rule::{Arg, MatchRuleError};
use std::convert::TryFrom;

#[test]
fn get_value() {
    let arg = Arg::try_from((0, "TEST".to_string())).unwrap();
    assert_eq!(arg.get_value(), "TEST");
}

#[test]
fn try_from_error() {
    let result = Arg::try_from((64, "TEST".to_string()));
    assert_eq!(result, Err(MatchRuleError::ArgIndexTooBig(64)));
}
