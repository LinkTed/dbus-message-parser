use dbus_message_parser::value::{Interface, InterfaceError};
use std::convert::TryFrom;

#[test]
fn interface_error_element_begin_digit_1() {
    let result = Interface::try_from("1");
    assert_eq!(result, Err(InterfaceError::ElementBeginDigit))
}

#[test]
fn interface_error_element_begin_digit_2() {
    let result = Interface::try_from("i1.1");
    assert_eq!(result, Err(InterfaceError::ElementBeginDigit))
}

#[test]
fn interface_error_element_begin_dot_1() {
    let result = Interface::try_from(".");
    assert_eq!(result, Err(InterfaceError::ElementBeginDot))
}

#[test]
fn interface_error_element_begin_dot_2() {
    let result = Interface::try_from("i.i1..");
    assert_eq!(result, Err(InterfaceError::ElementBeginDot))
}

#[test]
fn interface_error_end_dot() {
    let result = Interface::try_from("a.a.");
    assert_eq!(result, Err(InterfaceError::EndDot))
}

#[test]
fn interface_error_empty() {
    let result = Interface::try_from("");
    assert_eq!(result, Err(InterfaceError::Empty))
}

#[test]
fn interface_error_elements() {
    let result = Interface::try_from("a");
    assert_eq!(result, Err(InterfaceError::Elements))
}

#[test]
fn interface_error_exceed_maximum() {
    let result = Interface::try_from(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    assert_eq!(result, Err(InterfaceError::ExceedMaximum(256)))
}

#[test]
fn interface_error_invalid_char() {
    let result = Interface::try_from("/");
    assert_eq!(result, Err(InterfaceError::InvalidChar(b'/')))
}
