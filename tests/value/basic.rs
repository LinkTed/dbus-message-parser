use dbus_message_parser::{ObjectPath, ObjectPathError};
use std::convert::TryFrom;

#[test]
fn object_path_2() {
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
