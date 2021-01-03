use dbus_message_parser::value::{Bus, Error, Interface, Member, ObjectPath, Signature};
use std::convert::TryFrom;
use std::string::ToString;

#[test]
fn bus() {
    let bus = Bus::try_from(":1000").unwrap();
    assert_eq!(&bus.to_string(), ":1000");
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
    let signature = Signature::try_from("aii").unwrap();
    assert_eq!(&signature.to_string(), "aii");
}
