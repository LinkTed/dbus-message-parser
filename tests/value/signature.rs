use dbus_message_parser::value::{Type, TypeError};

#[test]
fn invalid_char_error() {
    let signature = Type::from_string_to_signature("w");
    assert_eq!(signature, Err(TypeError::InvalidChar(b'w')))
}

#[test]
fn array_depth_error() {
    let signature = Type::from_string_to_signature("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(signature, Err(TypeError::ArrayDepth(33)))
}

#[test]
fn struct_depth_error() {
    let signature = Type::from_string_to_signature("(((((((((((((((((((((((((((((((((");
    assert_eq!(signature, Err(TypeError::StructDepth(33)))
}

#[test]
fn dict_depth_error() {
    let signature = Type::from_string_to_signature("{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{");
    assert_eq!(signature, Err(TypeError::DictDepth(33)))
}

#[test]
fn too_big_error() {
    let signature = Type::from_string_to_signature(
        "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
    iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
    iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
    iiiiiiiiiii",
    );
    assert_eq!(signature, Err(TypeError::ExceedMaximum(256)))
}

#[test]
fn closing_curly_bracket_error() {
    let signature = Type::from_string_to_signature("{isi");
    assert_eq!(signature, Err(TypeError::ClosingCurlyBracket(3, b'i')))
}

#[test]
fn too_short() {
    let signature = Type::from_string_to_signature("{is");
    assert_eq!(signature, Err(TypeError::TooShort(3, 3)))
}
