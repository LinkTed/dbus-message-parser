use dbus_message_parser::value::{ObjectPath, ObjectPathError};
use std::convert::TryFrom;

#[test]
fn object_path_default() {
    let object_path = ObjectPath::default();
    assert_eq!(object_path.as_ref(), "/");
}

#[test]
fn object_path() {
    ObjectPath::try_from("/object/path").unwrap();
}

#[test]
fn object_path_into() {
    let object_path = ObjectPath::try_from("/object/path").unwrap();
    let string: String = object_path.into();
    assert_eq!(&string, "/object/path")
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

#[test]
fn object_path_error_begin_alphanumeric_and_underscore_and_hyphen() {
    let o = ObjectPath::try_from("object/path");
    assert_eq!(
        o,
        Err(ObjectPathError::BeginAlphanumericAndUnderscoreAndHyphen)
    );
}

#[test]
fn object_path_error_end_slash() {
    let o = ObjectPath::try_from("/object/path/");
    assert_eq!(o, Err(ObjectPathError::EndSlash));
}

#[test]
fn object_path_error_empty() {
    let o = ObjectPath::try_from("");
    assert_eq!(o, Err(ObjectPathError::Empty));
}

#[test]
fn object_path_error_element_empty_1() {
    let o = ObjectPath::try_from("//");
    assert_eq!(o, Err(ObjectPathError::ElementEmtpy));
}

#[test]
fn object_path_error_element_empty_2() {
    let o = ObjectPath::try_from("/object//");
    assert_eq!(o, Err(ObjectPathError::ElementEmtpy));
}

#[test]
fn object_path_error_invalid_char() {
    let o = ObjectPath::try_from("(");
    assert_eq!(o, Err(ObjectPathError::InvalidChar(b'(')));
}
