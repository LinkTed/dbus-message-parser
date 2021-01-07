use dbus_message_parser::value::{Member, MemberError};
use std::convert::TryFrom;

#[test]
fn member() {
    let member = Member::try_from("member").unwrap();
    assert_eq!(member.as_ref(), "member");
}

#[test]
fn member_error_empty() {
    let result = Member::try_from("");
    assert_eq!(result, Err(MemberError::Empty));
}

#[test]
fn member_error_begin_digit() {
    let result = Member::try_from("1");
    assert_eq!(result, Err(MemberError::BeginDigit));
}

#[test]
fn member_error_exceed_maximum() {
    let result = Member::try_from(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(result, Err(MemberError::ExceedMaximum(256)));
}

#[test]
fn member_error_invalid_char() {
    let result = Member::try_from("/");
    assert_eq!(result, Err(MemberError::InvalidChar(b'/')));
}
