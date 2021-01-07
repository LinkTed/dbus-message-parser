use dbus_message_parser::value::{Error, ErrorError};
use std::convert::TryFrom;

#[test]
fn error() {
    let error = Error::try_from("error.error").unwrap();
    let string: String = error.into();
    assert_eq!(&string, "error.error");
}

#[test]
fn error_error_element_begin_digit_1() {
    let result = Error::try_from("1");
    assert_eq!(result, Err(ErrorError::ElementBeginDigit))
}

#[test]
fn error_error_element_begin_digit_2() {
    let result = Error::try_from("error.1");
    assert_eq!(result, Err(ErrorError::ElementBeginDigit))
}

#[test]
fn error_error_element_begin_dot_1() {
    let result = Error::try_from(".");
    assert_eq!(result, Err(ErrorError::ElementBeginDot))
}

#[test]
fn error_error_element_begin_dot_2() {
    let result = Error::try_from("error..");
    assert_eq!(result, Err(ErrorError::ElementBeginDot))
}

#[test]
fn error_error_end_dot() {
    let result = Error::try_from("error1.error2.");
    assert_eq!(result, Err(ErrorError::EndDot))
}

#[test]
fn error_error_empty() {
    let result = Error::try_from("");
    assert_eq!(result, Err(ErrorError::Empty))
}

#[test]
fn error_error_elements() {
    let result = Error::try_from("error");
    assert_eq!(result, Err(ErrorError::Elements))
}

#[test]
fn error_error_exceed_maximum() {
    let result = Error::try_from(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(result, Err(ErrorError::ExceedMaximum(256)))
}

#[test]
fn error_error_invalid_char() {
    let result = Error::try_from("/");
    assert_eq!(result, Err(ErrorError::InvalidChar(b'/')))
}
