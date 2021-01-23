use crate::value::{Array, ArrayError, Type, Value};

#[test]
fn array_signature_mismatch() {
    let int_32 = Value::Int32(10);
    let int_16 = Value::Int16(10);
    let array = vec![int_32, int_16];
    let type_ = Type::Int32;

    assert_eq!(
        Array::new(array, type_),
        Err(ArrayError::TypeMismatch(Type::Int32, Type::Int16,)),
    );
}
