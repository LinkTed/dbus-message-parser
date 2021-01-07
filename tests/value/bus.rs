use dbus_message_parser::value::{Bus, BusError};
use std::convert::TryFrom;

#[test]
fn bus_error_begin_digit() {
    let result = Bus::try_from("1");
    assert_eq!(result, Err(BusError::BeginDigit))
}

#[test]
fn bus_error_begin_dot() {
    let result = Bus::try_from(".");
    assert_eq!(result, Err(BusError::BeginDot))
}

#[test]
fn bus_error_end_dot_1() {
    let result = Bus::try_from("bus.name.");
    assert_eq!(result, Err(BusError::EndDot))
}

#[test]
fn bus_error_end_dot_2() {
    let result = Bus::try_from(":1.");
    assert_eq!(result, Err(BusError::EndDot))
}

#[test]
fn bus_error_unique_empty() {
    let result = Bus::try_from(":");
    assert_eq!(result, Err(BusError::UniqueEmpty))
}

#[test]
fn bus_error_well_known_element_begin_digit() {
    let result = Bus::try_from("bus.1");
    assert_eq!(result, Err(BusError::WellKnownElementBeginDigit))
}

#[test]
fn bus_error_element_begin_dot_1() {
    let result = Bus::try_from("bus..");
    assert_eq!(result, Err(BusError::ElementBeginDot))
}

#[test]
fn bus_error_element_begin_dot_2() {
    let result = Bus::try_from(":.");
    assert_eq!(result, Err(BusError::ElementBeginDot))
}

#[test]
fn bus_error_element_begin_dot_3() {
    let result = Bus::try_from(":1..");
    assert_eq!(result, Err(BusError::ElementBeginDot))
}

#[test]
fn bus_error_element_begin_dot_4() {
    let result = Bus::try_from(":1.1..");
    assert_eq!(result, Err(BusError::ElementBeginDot))
}

#[test]
fn bus_error_colon_1() {
    let result = Bus::try_from("bus:");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_colon_2() {
    let result = Bus::try_from("::");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_colon_3() {
    let result = Bus::try_from(":11:");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_colon_4() {
    let result = Bus::try_from(":1.:");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_colon_5() {
    let result = Bus::try_from(":1.1:");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_colon_6() {
    let result = Bus::try_from("bus1.:");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_colon_7() {
    let result = Bus::try_from("bus1.bus:");
    assert_eq!(result, Err(BusError::Colon))
}

#[test]
fn bus_error_empty() {
    let result = Bus::try_from("");
    assert_eq!(result, Err(BusError::Empty))
}

#[test]
fn bus_error_elements_1() {
    let result = Bus::try_from("bus");
    assert_eq!(result, Err(BusError::Elements))
}

#[test]
fn bus_error_elements_2() {
    let result = Bus::try_from(":1");
    assert_eq!(result, Err(BusError::Elements))
}

#[test]
fn bus_error_exceed_maximum() {
    let result = Bus::try_from(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    assert_eq!(result, Err(BusError::ExceedMaximum(256)))
}

#[test]
fn bus_error_invalid_char() {
    let result = Bus::try_from("<");
    assert_eq!(result, Err(BusError::InvalidChar(b'<')))
}
