use dbus_message_parser::{ObjectPath, ObjectPathError};
use std::convert::TryFrom;

#[test]
fn object_path() {
    ObjectPath::try_from("/object/path").unwrap();
}

#[test]
fn object_path_error() {
    let o = ObjectPath::try_from("object/path");
    assert_eq!(
        Err(ObjectPathError::TryFromError("object/path".to_string())),
        o
    );
}

#[test]
fn object_path_start_with_1() {
    let base = ObjectPath::try_from("/object").unwrap();
    let path = ObjectPath::try_from("/object/path").unwrap();

    assert!(path.start_with(&base));
    assert!(!base.start_with(&base));
}

#[test]
fn object_path_start_with_2() {
    let base = ObjectPath::try_from("/object").unwrap();
    let path = ObjectPath::try_from("/object_/path").unwrap();

    assert!(!path.start_with(&base));
    assert!(!base.start_with(&base));
}

#[test]
fn object_path_start_with_3() {
    let base = ObjectPath::try_from("/").unwrap();
    let path = ObjectPath::try_from("/object/path").unwrap();

    assert!(path.start_with(&base));
    assert!(base.start_with(&base));
}
