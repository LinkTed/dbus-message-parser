use dbus_message_parser::{Header, Value};
use std::collections::BTreeSet;
use std::convert::TryFrom;

#[test]
fn test_header_1() {
    let b = Value::Byte(1);
    let v = Value::Variant(vec![Value::ObjectPath("/".to_string())]);
    let header = Header::try_from(Value::Struct(vec![b, v])).unwrap();

    let mut tree = BTreeSet::new();
    tree.insert(header);

    let b = Value::Byte(1);
    let v = Value::Variant(vec![Value::ObjectPath("/a/".to_string())]);
    let header = Header::try_from(Value::Struct(vec![b, v])).unwrap();

    let mut tree = BTreeSet::new();
    tree.insert(header);

    assert_eq!(tree.len(), 1);
}
