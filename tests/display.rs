use dbus_message_parser::value::{Bus, Error, Interface, Member, ObjectPath, Type};
use std::convert::TryFrom;
use std::string::ToString;

#[test]
fn bus() {
    let bus = Bus::try_from(":1.1000").unwrap();
    assert_eq!(&bus.to_string(), ":1.1000");
}

#[test]
fn error() {
    let error = Error::try_from("this.is.an.error").unwrap();
    assert_eq!(&error.to_string(), "this.is.an.error");
}

#[test]
fn interface() {
    let interface = Interface::try_from("this.is.an.interface").unwrap();
    assert_eq!(&interface.to_string(), "this.is.an.interface");
}

#[test]
fn member() {
    let member = Member::try_from("Member").unwrap();
    assert_eq!(&member.to_string(), "Member");
}

#[test]
fn object_path() {
    let object_path = ObjectPath::try_from("/object/path").unwrap();
    assert_eq!(&object_path.to_string(), "/object/path");
}

#[test]
fn signature() {
    let signature = Type::try_from("aai").unwrap();
    assert_eq!(&signature.to_string(), "aai");
}

#[test]
fn type_byte() {
    assert_eq!(&Type::Byte.to_string(), "y");
}

#[test]
fn type_boolen() {
    assert_eq!(&Type::Boolean.to_string(), "b");
}

#[test]
fn type_int16() {
    assert_eq!(&Type::Int16.to_string(), "n");
}

#[test]
fn type_uint16() {
    assert_eq!(&Type::Uint16.to_string(), "q");
}

#[test]
fn type_int32() {
    assert_eq!(&Type::Int32.to_string(), "i");
}

#[test]
fn type_uint32() {
    assert_eq!(&Type::Uint32.to_string(), "u");
}

#[test]
fn type_int64() {
    assert_eq!(&Type::Int64.to_string(), "x");
}

#[test]
fn type_uint64() {
    assert_eq!(&Type::Uint64.to_string(), "t");
}

#[test]
#[cfg(target_family = "unix")]
fn type_unix_fd() {
    assert_eq!(&Type::UnixFD.to_string(), "h");
}

#[test]
fn type_double() {
    assert_eq!(&Type::Double.to_string(), "d");
}

#[test]
fn type_string() {
    assert_eq!(&Type::String.to_string(), "s");
}

#[test]
fn type_object_path() {
    assert_eq!(&Type::ObjectPath.to_string(), "o");
}

#[test]
fn type_variant() {
    assert_eq!(&Type::Variant.to_string(), "v");
}

#[test]
fn type_signature() {
    assert_eq!(&Type::Signature.to_string(), "g");
}

#[test]
fn type_array() {
    let type_ = Type::Int32;
    let type_ = Type::Array(Box::new(type_));
    assert_eq!(&type_.to_string(), "ai");
}

#[test]
fn type_struct() {
    let type_ = Type::Int32;
    let type_ = Type::Struct(vec![type_]);
    assert_eq!(&type_.to_string(), "(i)");
}

#[test]
fn type_dict_entry() {
    let key = Type::Int32;
    let value = Type::String;
    let type_ = Type::DictEntry(Box::new((key, value)));
    assert_eq!(&type_.to_string(), "{is}");
}
