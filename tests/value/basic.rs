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
fn object_path_starts_with_1() {
    let base = ObjectPath::try_from("/object").unwrap();
    let path = ObjectPath::try_from("/object/path").unwrap();

    assert!(path.starts_with(&base));
    assert!(!base.starts_with(&base));
}

#[test]
fn object_path_starts_with_2() {
    let base = ObjectPath::try_from("/object").unwrap();
    let path = ObjectPath::try_from("/object_/path").unwrap();

    assert!(!path.starts_with(&base));
    assert!(!base.starts_with(&base));
}

#[test]
fn object_path_starts_with_3() {
    let base = ObjectPath::try_from("/").unwrap();
    let path = ObjectPath::try_from("/object/path").unwrap();

    assert!(path.starts_with(&base));
    assert!(!base.starts_with(&base));
}

#[test]
fn object_path_strip_prefix_elements_1() {
    let base = ObjectPath::try_from("/object").unwrap();
    let path = ObjectPath::try_from("/object/path").unwrap();

    let path_base_vec: Vec<&str> = path.strip_prefix_elements(&base).unwrap().collect();

    assert_eq!(path_base_vec, vec!["path"]);
    assert!(base.strip_prefix_elements(&base).is_none());
}

#[test]
fn object_path_strip_prefix_elements_2() {
    let base = ObjectPath::try_from("/object").unwrap();
    let path = ObjectPath::try_from("/object_/path").unwrap();

    assert!(path.strip_prefix_elements(&base).is_none());
    assert!(base.strip_prefix_elements(&base).is_none());
}

#[test]
fn object_path_strip_prefix_elements_3() {
    let base = ObjectPath::try_from("/").unwrap();
    let path = ObjectPath::try_from("/object/path").unwrap();

    let path_base_vec: Vec<&str> = path.strip_prefix_elements(&base).unwrap().collect();

    assert_eq!(path_base_vec, vec!["object", "path"]);
    assert!(base.strip_prefix_elements(&base).is_none());
}
