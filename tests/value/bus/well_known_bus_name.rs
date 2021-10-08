use dbus_message_parser::value::{Bus, BusError, UniqueConnectionNameError, WellKnownBusNameError};
use std::convert::TryFrom;

#[test]
fn bus_error_begin_digit() {
    let result = Bus::try_from("1");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::BeginDigit
        ))
    )
}

#[test]
fn bus_error_begin_dot() {
    let result = Bus::try_from(".");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::BeginDot
        ))
    )
}

#[test]
fn bus_error_end_dot_1() {
    let result = Bus::try_from("bus.name.");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::EndDot
        ))
    )
}

#[test]
fn bus_error_end_dot_2() {
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
fn bus_error_well_known_element_begin_digit() {
    let result = Bus::try_from("bus.1");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::ElementBeginDigit
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_1() {
    let result = Bus::try_from("bus..");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::ElementBeginDot
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_2() {
    let result = Bus::try_from(":.");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::ElementBeginDot
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_3() {
    let result = Bus::try_from(":1..");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::ElementBeginDot
        ))
    )
}

#[test]
fn bus_error_element_begin_dot_4() {
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
    let result = Bus::try_from("bus:");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::InvalidChar(b':')
        ))
    )
}

#[test]
fn bus_error_colon_2() {
    let result = Bus::try_from("::");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_3() {
    let result = Bus::try_from(":11:");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_4() {
    let result = Bus::try_from(":1.:");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_5() {
    let result = Bus::try_from(":1.1:");
    assert_eq!(
        result,
        Err(BusError::UniqueConnectionNameError(
            UniqueConnectionNameError::Colon
        ))
    )
}

#[test]
fn bus_error_colon_6() {
    let result = Bus::try_from("bus1.:");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::InvalidChar(b':')
        ))
    )
}

#[test]
fn bus_error_colon_7() {
    let result = Bus::try_from("bus1.bus:");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::InvalidChar(b':')
        ))
    )
}

#[test]
fn bus_error_empty() {
    let result = Bus::try_from("");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::Empty
        ))
    )
}

#[test]
fn bus_error_elements_1() {
    let result = Bus::try_from("bus");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::Elements
        ))
    )
}

#[test]
fn bus_error_elements_2() {
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
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::ExceedMaximum(256)
        ))
    )
}

#[test]
fn bus_error_invalid_char() {
    let result = Bus::try_from("<");
    assert_eq!(
        result,
        Err(BusError::WellKnownBusNameError(
            WellKnownBusNameError::InvalidChar(b'<')
        ))
    )
}
