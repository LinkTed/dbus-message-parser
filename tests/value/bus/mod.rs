mod unique_connection_name;
mod well_known_bus_name;

use dbus_message_parser::value::Bus;
use std::convert::TryFrom;

#[test]
fn bus_try_from_1() {
    let result = Bus::try_from(&b"a.a"[..]);
    assert!(result.is_ok());
}

#[test]
fn bus_try_from_2() {
    let result = Bus::try_from(&b":1.1"[..]);
    assert!(result.is_ok());
}

#[test]
fn bus_as_ref_1() {
    let result = Bus::try_from(&b"a.a"[..]);
    assert_eq!(result.unwrap().as_ref(), "a.a");
}

#[test]
fn bus_as_ref_2() {
    let result = Bus::try_from(&b":1.1"[..]);
    assert_eq!(result.unwrap().as_ref(), ":1.1");
}

#[test]
fn bus_partial_eq_1() {
    let result = Bus::try_from(&b":1.1"[..]);
    assert_eq!(&result.unwrap(), ":1.1");
}

#[test]
fn bus_partial_eq_2() {
    let result = Bus::try_from(&b":1.1"[..]);
    assert_ne!(&result.unwrap(), "a.a");
}

#[test]
fn bus_partial_eq_3() {
    let result = Bus::try_from(&b"a.a"[..]);
    assert_eq!(&result.unwrap(), "a.a");
}

#[test]
fn bus_partial_eq_4() {
    let result = Bus::try_from(&b"a.a"[..]);
    assert_ne!(&result.unwrap(), ":1.1");
}
