use std::mem::size_of;
use bytes::{BytesMut, BufMut};
use crate::value::Value;


/// An enum representing all errors, which can occur during the encoding.
#[derive(Debug)]
pub enum EncodeError {
    ObjectPathInvalid(String),
    ArraySignatureMismatch(String, String),
    UnknownSignature(String),
    NullSignature,
    SignatureTooLarge(String)
}

/// This is a helper function to add the algin to the buffer.
pub fn encode_algin(buf: &mut BytesMut, a: usize) {
    let p = buf.len() % a;
    if p != 0 {
        let mut p = a - p;
        buf.reserve(p);

        while p != 0 {
            buf.put_u8(0);
            p -= 1;
        }
    }
}

/// Encode a `u8` into the buffer.
pub fn byte(buf: &mut BytesMut, b: u8) -> Result<(), EncodeError> {
    buf.reserve( size_of::<u8>());
    buf.put_u8(b);

    Ok(())
}

/// Encode a `bool` into the buffer.
pub fn boolean(buf: &mut BytesMut, b: bool, is_le: bool)
    -> Result<(), EncodeError> {
    buf.reserve( size_of::<u32>());
    if is_le {
        buf.put_u32_le(b as u32);
    } else {
        buf.put_u32(b as u32);
    }

    Ok(())
}

/// Encode a `i16` into the buffer.
pub fn i16(buf: &mut BytesMut, i: i16, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<i16>());
    if is_le {
        buf.put_i16_le(i);
    } else {
        buf.put_i16(i);
    }

    Ok(())
}

/// Encode a `u16` into the buffer.
pub fn u16(buf: &mut BytesMut, u: u16, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<u16>());
    if is_le {
        buf.put_u16_le(u);
    } else {
        buf.put_u16(u);
    }

    Ok(())
}

/// Encode a `i32` into the buffer.
pub fn i32(buf: &mut BytesMut, i: i32, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<i32>());
    if is_le {
        buf.put_i32_le(i);
    } else {
        buf.put_i32(i);
    }

    Ok(())
}

/// Encode a `u32` into the buffer.
pub fn u32(buf: &mut BytesMut, u: u32, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<u32>());
    if is_le {
        buf.put_u32_le(u);
    } else {
        buf.put_u32(u);
    }

    Ok(())
}

/// Encode a `i64` into the buffer.
pub fn i64(buf: &mut BytesMut, i: i64, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<i64>());
    if is_le {
        buf.put_i64_le(i);
    } else {
        buf.put_i64(i);
    }

    Ok(())
}

/// Encode a `u64` into the buffer.
pub fn u64(buf: &mut BytesMut, u: u64, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<u64>());
    if is_le {
        buf.put_u64_le(u);
    } else {
        buf.put_u64(u);
    }

    Ok(())
}

/// Encode a `f64` into the buffer.
pub fn f64(buf: &mut BytesMut, f: f64, is_le: bool) -> Result<(), EncodeError> {
    buf.reserve( size_of::<f64>());
    if is_le {
        buf.put_f64_le(f);
    } else {
        buf.put_f64(f);
    }

    Ok(())
}

/// Encode a `&str` into the buffer and use 4 bytes.
pub fn str(buf: &mut BytesMut, s: &str, is_le: bool)
    -> Result<(), EncodeError> {
    let string_len = s.len();
    buf.reserve( size_of::<u32>() + string_len + 1);
    if is_le {
        buf.put_u32_le(string_len as u32);
    } else {
        buf.put_u32(string_len as u32);
    }

    buf.put(s.as_bytes());
    buf.put_u8(0);

    Ok(())
}

/// Encode a `&str` into the buffer and use 1 bytes.
pub fn sig(buf: &mut BytesMut, s: &str) -> Result<(), EncodeError> {
    let sig_len = s.len();
    if (u8::max_value() as usize) < sig_len {
        return Err(EncodeError::SignatureTooLarge(s.to_string()))
    }
    buf.reserve( size_of::<u8>() + sig_len + 1);
    buf.put_u8(sig_len as u8);
    buf.put(s.as_bytes());
    buf.put_u8(0);

    Ok(())
}

/// Encode a `&Vec<Value>` as an array into the buffer.
pub fn array(buf: &mut BytesMut, vec: &Vec<Value>, sig: &str, is_le: bool)
    -> Result<(), EncodeError> {
    let mut array_buf = BytesMut::with_capacity(128);
    let mut sig_cmp = String::new();
    for v in vec {
        v.get_signature(&mut sig_cmp);
        if sig == sig_cmp {
            sig_cmp.clear();
        } else {
            return Err(EncodeError::ArraySignatureMismatch(sig.to_string(),
                                                           sig_cmp.clone()))
        }
        v.encode(&mut array_buf, is_le)?;
    }

    let array_len = array_buf.len();
    buf.reserve( size_of::<u32>());
    if is_le {
        buf.put_u32_le(array_len as u32);
    } else {
        buf.put_u32(array_len as u32);
    }

    match sig.get(0..1) {
        Some(s) => {
            match s {
                "v" | "y" | "g" => {},
                "n" | "q" => encode_algin(buf, 2),
                "b" | "i" | "u" | "a" | "s" | "o" =>  encode_algin(buf, 4),
                "x" | "t" | "d" | "(" | "{" => encode_algin(buf, 8),
                signature => return Err(EncodeError::UnknownSignature(
                    signature.to_string()))
            }
        },
        None => return Err(EncodeError::NullSignature)
    }

    buf.reserve(array_len);
    buf.extend(array_buf);

    Ok(())
}

/// Encode a `&Vec<Value>` as a struct into the buffer.
pub fn encode_struct(buf: &mut BytesMut, vec: &Vec<Value>, is_le: bool)
    -> Result<(), EncodeError> {
    for v in vec {
        v.encode(buf, is_le)?;
    }

    Ok(())
}

/// Encode a `&Box<(Value, Value)>` as a dict entry into the buffer.
pub fn dict_entry(buf: &mut BytesMut, b: &Box<(Value, Value)>, is_le: bool)
    -> Result<(), EncodeError> {
    let (key, value) = &**b;
    key.encode(buf, is_le)?;
    value.encode(buf, is_le)
}

/// Encode a `&Vec<Value>` as a variant into the buffer.
pub fn variant(buf: &mut BytesMut, vec: &Vec<Value>, is_le: bool)
    -> Result<(), EncodeError> {
    let mut signature = String::new();

    for v in vec {
        v.get_signature(&mut signature);
    }

    sig(buf, &signature)?;

    for v in vec {
        v.encode(buf, is_le)?;
    }

    Ok(())
}
