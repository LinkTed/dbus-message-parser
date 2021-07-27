use dbus_message_parser::value::{Array, Struct, StructError, Type, Value};
use std::convert::TryFrom;

#[test]
fn byte_alignment() {
    let type_ = Type::Byte;
    assert_eq!(type_.get_alignment(), 1);
}

#[test]
fn boolen_alignment() {
    let type_ = Type::Boolean;
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn int16_alignment() {
    let type_ = Type::Int16;
    assert_eq!(type_.get_alignment(), 2);
}

#[test]
fn uint16_alignment() {
    let type_ = Type::Uint16;
    assert_eq!(type_.get_alignment(), 2);
}

#[test]
fn int32_alignment() {
    let type_ = Type::Int32;
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn uint32_alignment() {
    let type_ = Type::Uint32;
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn int64_alignment() {
    let type_ = Type::Int64;
    assert_eq!(type_.get_alignment(), 8);
}

#[test]
fn uint64_alignment() {
    let type_ = Type::Uint64;
    assert_eq!(type_.get_alignment(), 8);
}

#[test]
fn double_alignment() {
    let type_ = Type::Double;
    assert_eq!(type_.get_alignment(), 8);
}

#[test]
fn string_alignment() {
    let type_ = Type::String;
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn object_path_alignment() {
    let type_ = Type::ObjectPath;
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn signature_alignment() {
    let type_ = Type::Signature;
    assert_eq!(type_.get_alignment(), 1);
}

#[test]
fn array_alignment() {
    let signature = Type::Int32;
    let type_ = Type::Array(Box::new(signature));
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn struct_alignment() {
    let signature = Type::Int32;
    let type_ = Type::Struct(vec![signature]);
    assert_eq!(type_.get_alignment(), 8);
}

#[test]
fn dict_entry_alignment() {
    let signature_key = Type::Int32;
    let signature_value = Type::String;
    let type_ = Type::DictEntry(Box::new((signature_key, signature_value)));
    assert_eq!(type_.get_alignment(), 8);
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd_alignment() {
    let type_ = Type::UnixFD;
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn array_into() {
    let array = vec![Value::Int32(2)];
    let array = Array::new(array, Type::Int32).unwrap();
    let array: Vec<Value> = array.into();
    assert_eq!(array, vec![Value::Int32(2)]);
}

#[test]
fn struct_error() {
    let result = Struct::try_from(Vec::new());
    assert_eq!(result, Err(StructError::Empty));
}
