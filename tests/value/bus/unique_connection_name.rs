use dbus_message_parser::value::{Bus, BusError, UniqueConnectionName, UniqueConnectionNameError};
use std::convert::TryFrom;

#[test]
fn unique_connection_name_error_begin_digit() {
    let result = UniqueConnectionName::try_from("1");
    assert_eq!(result, Err(UniqueConnectionNameError::BeginColon))
}

#[test]
fn unique_connection_name_error_begin_dot() {
    let result = UniqueConnectionName::try_from(".");
    assert_eq!(result, Err(UniqueConnectionNameError::BeginDot))
}

#[test]
fn bus_error_end_dot() {
    let result = Bus::try_from(":1.");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::EndDot
        ))
    )
}

#[test]
fn bus_error_unique_empty() {
    let result = Bus::try_from(":");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Empty
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_1() {
    let result = Bus::try_from(":.");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::ElementBeginDot
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_2() {
    let result = Bus::try_from(":1..");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::ElementBeginDot
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_3() {
    let result = Bus::try_from(":1.1..");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::ElementBeginDot
        ))
    )
}

#[test]
fn bus_error_colon_1() {
    let result = Bus::try_from("::");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_2() {
    let result = Bus::try_from(":11:");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_3() {
    let result = Bus::try_from(":1.:");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_4() {
    let result = Bus::try_from(":1.1:");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn unique_connection_name_error_empty() {
    let result = UniqueConnectionName::try_from("");
    assert_eq!(result, Err(UniqueConnectionNameError::Empty))
}

#[test]
fn bus_error_elements() {
    let result = Bus::try_from(":1");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Elements
        ))
    )
}

#[test]
fn bus_error_exceed_maximum() {
    let result = Bus::try_from(
        ":aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::ExceedMaximum(256)
        ))
    )
}

#[test]
fn bus_error_invalid_char() {
    let result = Bus::try_from(":<");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::InvalidChar(b'<')
        ))
    )
}

#[test]
fn unique_connection_name_try_from() {
    let result = UniqueConnectionName::try_from(&b":1.1"[..]);
    assert!(result.is_ok())
}

#[test]
fn unique_connection_name_as_ref() {
    let result = UniqueConnectionName::try_from(&b":1.1"[..]);
    assert_eq!(result.unwrap().as_ref(), ":1.1")
}

#[test]
fn unique_connection_name_partial_eq() {
    let result = UniqueConnectionName::try_from(&b":1.1"[..]);
    assert_eq!(&result.unwrap(), ":1.1")
}
