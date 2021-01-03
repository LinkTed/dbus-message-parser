use dbus_message_parser::value::{Signature, SignatureError, Value};
use std::convert::TryFrom;

#[test]
fn invalid_char_error() {
    let signature = Signature::try_from("w");
    assert_eq!(signature, Err(SignatureError::InvalidChar('w')))
}

#[test]
fn array_depth_error() {
    let signature = Signature::try_from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(signature, Err(SignatureError::ArrayDepth(33)))
}

#[test]
fn struct_depth_error() {
    let signature = Signature::try_from("(((((((((((((((((((((((((((((((((");
    assert_eq!(signature, Err(SignatureError::StructDepth(33)))
}

#[test]
fn dict_depth_error() {
    let signature = Signature::try_from("{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{");
    assert_eq!(signature, Err(SignatureError::DictDepth(33)))
}

#[test]
fn too_big_error() {
    let signature = Signature::try_from(
        "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
    iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
    iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
    iiiiiiiiiii",
    );
    assert_eq!(signature, Err(SignatureError::TooBig(256)))
}

#[test]
fn closing_curly_bracket_error() {
    let signature = Signature::try_from("{isi");
    assert_eq!(signature, Err(SignatureError::ClosingCurlyBracket(3, 'i')))
}

#[test]
fn too_short() {
    let signature = Signature::try_from("{is");
    assert_eq!(signature, Err(SignatureError::TooShort(3, 3)))
}

#[test]
fn new() {
    let int_32 = Value::Int32(0);
    let string = Value::String(String::new());
    let values = vec![int_32, string];
    let signature = Signature::new(&values).unwrap();
    assert_eq!(&signature, "is")
}
