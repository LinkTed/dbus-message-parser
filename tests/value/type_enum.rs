use dbus_message_parser::value::{Type, TypeError};
use std::convert::TryFrom;

#[test]
fn type_error_exceed_maximum_1() {
    let result = Type::try_from(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(result, Err(TypeError::ExceedMaximum(256)));
}

#[test]
fn type_error_exceed_maximum_2() {
    let signature = vec![Type::Int32; 256];
    let result = Type::from_signature_to_string(&signature[..]);
    assert_eq!(result, Err(TypeError::ExceedMaximum(256)));
}
