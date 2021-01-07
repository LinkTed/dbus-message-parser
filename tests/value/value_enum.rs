use dbus_message_parser::value::{Signature, Type};
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
    let signature = Signature::try_from("i").unwrap();
    let type_ = Type::Array(signature);
    assert_eq!(type_.get_alignment(), 4);
}

#[test]
fn struct_alignment() {
    let signature = Signature::try_from("i").unwrap();
    let type_ = Type::Struct(signature);
    assert_eq!(type_.get_alignment(), 8);
}

#[test]
fn dict_entry_alignment() {
    let signature_key = Signature::try_from("i").unwrap();
    let signature_value = Signature::try_from("s").unwrap();
    let type_ = Type::DictEntry(signature_key, signature_value);
    assert_eq!(type_.get_alignment(), 8);
}

#[cfg(target_family = "unix")]
#[test]
fn unix_fd_alignment() {
    let type_ = Type::UnixFD;
    assert_eq!(type_.get_alignment(), 4);
}
