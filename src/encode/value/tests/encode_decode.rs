use crate::decode::Decoder;
use crate::encode::Encoder;
use crate::value::{Array, Type, Value};
use std::cmp::Ordering;

fn encode_decode(value_1: &Value, is_le: bool) {
    let mut encoder = Encoder::new();
    encoder.value(value_1, is_le).unwrap();

    let bytes = encoder.buf.freeze();
    let mut decoder = Decoder::new(bytes);
    let type_ = value_1.get_type().unwrap();
    let value_2 = decoder.value(is_le, 0, &[type_]).unwrap();

    if let Some(ordering) = value_1.partial_cmp(&value_2[0]) {
        match ordering {
            Ordering::Equal => {}
            ordering => {
                panic!(
                    "\n{:?}\nleft:  {:?}\nright: {:?}",
                    ordering, value_1, value_2
                );
            }
        }
    }
}

#[test]
fn array() {
    let array_type = Type::from_string_to_signature("{s(bgav)}")
        .unwrap()
        .pop()
        .unwrap();
    let array = Array::new(Vec::new(), array_type).unwrap();
    let non_empty_variant = Value::Variant(Box::new(Value::Array(array)));
    let variant_signature = Type::Variant;
    let array = Array::new(vec![non_empty_variant], variant_signature).unwrap();
    let array = Value::Array(array);

    encode_decode(&array, true);
    encode_decode(&array, false);
}
