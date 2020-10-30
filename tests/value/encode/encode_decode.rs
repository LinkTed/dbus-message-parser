use bytes::BytesMut;
use dbus_message_parser::{Decoder, Encoder, Value};
use std::cmp::Ordering;

fn encode_decode(value_1: &Value, is_le: bool) {
    let mut bytes = BytesMut::new();
    let mut fds = Vec::new();
    let mut encoder = Encoder::new(&mut bytes, &mut fds);
    encoder.value(value_1, is_le).unwrap();

    let mut decoder = Decoder::new(&bytes);
    let value_2 = decoder.value(is_le, "av").unwrap();

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
    let empty_variant = Value::Variant(Vec::new());
    let non_empty_variant = Value::Variant(vec![Value::Array(Vec::new(), "{s(bgav)}".to_string())]);
    let array = Value::Array(
        vec![
            empty_variant.clone(),
            non_empty_variant.clone(),
            empty_variant.clone(),
            empty_variant.clone(),
        ],
        "v".to_string(),
    );

    encode_decode(&array, true);
    encode_decode(&array, false);
}
