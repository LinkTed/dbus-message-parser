use dbus_message_parser::message::MessageHeaderField;
use dbus_message_parser::value::{Bus, Error, Interface, Member, ObjectPath, Type};
use std::convert::{TryFrom, TryInto};
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

#[test]
fn message_header_field_path() {
    let message_header_field = MessageHeaderField::Path("/object/path".try_into().unwrap());
    assert_eq!(&message_header_field.to_string(), "path='/object/path'");
}

#[test]
fn message_header_field_interface() {
    let message_header_field = MessageHeaderField::Interface("interface.name".try_into().unwrap());
    assert_eq!(
        &message_header_field.to_string(),
        "interface='interface.name'"
    );
}

#[test]
fn message_header_field_member() {
    let message_header_field = MessageHeaderField::Member("MemberName".try_into().unwrap());
    assert_eq!(&message_header_field.to_string(), "member='MemberName'");
}

#[test]
fn message_header_field_error_name() {
    let message_header_field = MessageHeaderField::ErrorName("error.name".try_into().unwrap());
    assert_eq!(&message_header_field.to_string(), "error_name='error.name'");
}

#[test]
fn message_header_field_reply_serial() {
    let message_header_field = MessageHeaderField::ReplySerial(1234);
    assert_eq!(&message_header_field.to_string(), "reply_serial='1234'");
}

#[test]
fn message_header_field_destination() {
    let message_header_field =
        MessageHeaderField::Destination("destination.name".try_into().unwrap());
    assert_eq!(
        &message_header_field.to_string(),
        "destination='destination.name'"
    );
}

#[test]
fn message_header_field_sender() {
    let message_header_field = MessageHeaderField::Sender("sender.name".try_into().unwrap());
    assert_eq!(&message_header_field.to_string(), "sender='sender.name'");
}

#[test]
fn message_header_field_signature() {
    let message_header_field = MessageHeaderField::Signature(vec![Type::Byte]);
    assert_eq!(&message_header_field.to_string(), "signature='y'");
}

#[cfg(target_family = "unix")]
#[test]
fn message_header_field_unix_fds() {
    let message_header_field = MessageHeaderField::UnixFDs(2);
    assert_eq!(&message_header_field.to_string(), "unix_fds='2'");
}
