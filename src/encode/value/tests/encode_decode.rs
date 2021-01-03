use crate::decode::Decoder;
use crate::encode::Encoder;
use crate::value::Value;
use std::cmp::Ordering;
use std::convert::TryInto;

fn encode_decode(value_1: &Value, is_le: bool) {
    let mut encoder = Encoder::new();
    encoder.value(value_1, is_le).unwrap();

    let bytes = encoder.buf.freeze();
    let mut decoder = Decoder::new(bytes);
    let signature = value_1.get_signature().unwrap();
    let value_2 = decoder.value(is_le, 0, &signature).unwrap();

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
    let array_signature = "{s(bgav)}".try_into().unwrap();
    let non_empty_variant = Value::Variant(Box::new(Value::Array(Vec::new(), array_signature)));
    let variant_signature = "v".try_into().unwrap();
    let array = Value::Array(vec![non_empty_variant], variant_signature);

    encode_decode(&array, true);
    encode_decode(&array, false);
}
