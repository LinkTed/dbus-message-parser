use std::mem::size_of;
use std::string::FromUtf8Error;
use bytes::{Buf, BytesMut};
use crate::OBJECT_PATH_REGEX;
use crate::value::Value;


/// An enum representing all errors, which can occur during the decoding.
#[derive(Debug)]
pub enum DecodeError {
    TooShort,
    Utf8Error(FromUtf8Error),
    StringNotNull,
    ObjectPathRegex,
    InterfaceRegex,
    MemberRegex,
    BusNamesRegex,
    Signature,
    SignatureTooBig,
    Padding,
    ArrayTooBig,
    ArrayVecLen,
    ArrayLen,
    Endianness,
    Error,
    Header,
    MessageType,
    MessageFlags,
    BodySignatureMissing,
    DictVecLen,
    ArrayRecursion,
    StructRecursion,
}

/// Decode from a byte array at a specific offset to a `Value::Byte`.
pub fn byte(buf: &BytesMut, offset: &mut usize) -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<u8>();

    if let Some(buf) = buf.get(start..*offset) {
        Ok(Value::Byte(buf[0]))
    } else {
        Err(DecodeError::TooShort)
    }
}

/// Decode from a byte array at a specific offset to a `Value::Boolean`.
pub fn boolean(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<u32>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            buf.get_u32_le()
        } else {
            buf.get_u32()
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(Value::Boolean(r == 1))
}

/// Decode from a byte array at a specific offset to a `Value::Int16`.
pub fn int16(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<i16>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Int16(buf.get_i16_le())
        } else {
            Value::Int16(buf.get_i16())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `Value::Uint16`.
pub fn uint16(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<u16>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Uint16(buf.get_u16_le())
        } else {
            Value::Uint16(buf.get_u16())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `Value::Int32`.
pub fn int32(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<i32>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Int32(buf.get_i32_le())
        } else {
            Value::Int32(buf.get_i32())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `Value::Uint32`.
pub fn uint32(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<u32>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Uint32(buf.get_u32_le())
        } else {
            Value::Uint32(buf.get_u32())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `Value::Int64`.
pub fn int64(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<i64>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Int64(buf.get_i64_le())
        } else {
            Value::Int64(buf.get_i64())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `Value::Uint64`.
pub fn uint64(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<u64>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Uint64(buf.get_u64_le())
        } else {
            Value::Uint64(buf.get_u64())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `Value::Double`.
pub fn double(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let start = *offset;
    *offset += size_of::<f64>();

    let r = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            Value::Double(buf.get_f64_le())
        } else {
            Value::Double(buf.get_f64())
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    Ok(r)
}

/// Decode from a byte array at a specific offset to a `String`.
/// The size of the length is 4.
pub fn str(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<String, DecodeError> {
    let mut start = *offset;
    *offset += size_of::<u32>();

    let string_length = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            buf.get_u32_le() as usize
        } else {
            buf.get_u32() as usize
        }
    } else {
        return Err(DecodeError::TooShort);
    };

    start = *offset;
    *offset += string_length + 1;

    if let Some(buf) = buf.get(start..*offset)  {
        let string = String::from_utf8(buf[..string_length].to_vec())?;
        if *buf.last().unwrap() == 0 {
            Ok(string)
        } else {
            Err(DecodeError::StringNotNull)
        }
    } else {
        Err(DecodeError::TooShort)
    }
}

/// Decode from a byte array at a specific offset to a `Value::String`.
pub fn string(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    Ok(Value::String(str(buf, offset, is_le)?))
}

/// Decode from a byte array at a specific offset to a `Value::ObjectPath`.
pub fn path(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let s = str(buf, offset, is_le)?;

    if OBJECT_PATH_REGEX.is_match(&s) {
        Ok(Value::ObjectPath(s))
    } else {
        Err(DecodeError::ObjectPathRegex)
    }
}

/// Decode from a byte array at a specific offset to a `String`.
/// The size of the length is 1.
pub fn sig(buf: &BytesMut, offset: &mut usize) -> Result<String, DecodeError> {
    let mut start = *offset;
    *offset += size_of::<u8>();

    let string_size = if let Some(buf) = buf.get(start..*offset) {
        buf[0] as usize
    } else {
        return Err(DecodeError::TooShort)
    };

    start = *offset;
    *offset += string_size + 1;

    if let Some(buf)  = buf.get(start..*offset) {
        let string = String::from_utf8(buf[..string_size].to_vec())?;
        if *buf.last().unwrap() == 0 {
            Ok(string)
        } else {
            Err(DecodeError::StringNotNull)
        }
    } else {
        Err(DecodeError::TooShort)
    }
}

/// Decode from a byte array at a specific offset to a `Value::Signature`.
pub fn signature(buf: &BytesMut, offset: &mut usize)
    -> Result<Value, DecodeError> {
    Ok(Value::Signature(sig(buf, offset)?))
}

/// Decode from a byte array at a specific offset to a `Value::Variant`.
pub fn variant(buf: &BytesMut, offset: &mut usize, is_le: bool)
    -> Result<Value, DecodeError> {
    let signature = sig(buf, offset)?;

    Ok(Value::Variant(Value::decode(buf, offset, is_le, 0, 0, &signature)?))
}

/// This is a helper function to add the algin to the offset.
pub fn decode_algin(buf: &BytesMut, offset: &mut usize, a: usize)
    -> Result<(), DecodeError> {
    while *offset % a != 0 {
        if let Some(b) = buf.get(*offset) {
            if *b != 0 {
                return Err(DecodeError::Padding);
            }
        } else {
            return Err(DecodeError::TooShort);
        }
        *offset += 1;
    }
    Ok(())
}

/// Decode from a byte array at a specific offset to a `Value::Array`.
pub fn array(buf: &BytesMut, offset: &mut usize, is_le: bool,
             mut array_recursion: u8, struct_recursion: u8, signature: &str)
    -> Result<Value, DecodeError> {
    array_recursion += 1;

    let start = *offset;
    *offset += size_of::<u32>();

    let array_size = if let Some(mut buf) = buf.get(start..*offset) {
        if is_le {
            buf.get_u32_le()
        } else {
            buf.get_u32()
        }
    } else {
        return Err(DecodeError::TooShort)
    };

    if 67108864 < array_size {
        return Err(DecodeError::ArrayTooBig)
    }

    match signature.get(0..1) {
        Some(s) => {
            match s {
                "v" | "y" | "g" => {},
                "n" | "q" => decode_algin(buf, offset, 2)?,
                "b" | "i" | "u" | "a" | "s" | "o" =>
                    decode_algin(buf, offset, 4)?,
                "x" | "t" | "(" | "{" => decode_algin(buf, offset, 8)?,
                _ => return Err(DecodeError::Signature)
            }
        },
        None => return Err(DecodeError::Signature)
    };

    let mut r = Vec::new();

    let end = *offset + array_size as usize;
    while *offset < end {
        let mut v = Value::decode(buf, offset, is_le, array_recursion,
                                  struct_recursion, signature)?;
        if v.len() == 1 {
            r.push(v.pop().unwrap());
        } else {
            return Err(DecodeError::ArrayVecLen)
        }
    }

    if *offset == end {
        Ok(Value::Array(r, String::from(signature)))
    } else {
        Err(DecodeError::ArrayLen)
    }
}

/// Decode from a byte array at a specific offset to a `Value::Struct`.
pub fn decode_struct(buf: &BytesMut, offset: &mut usize, is_le: bool,
                     array_recursion: u8, mut struct_recursion: u8,
                     signature: &str) -> Result<Value, DecodeError> {
    struct_recursion += 1;
    Ok(Value::Struct(Value::decode(buf, offset, is_le,  array_recursion,
                                   struct_recursion, signature)?))
}

/// Decode from a byte array at a specific offset to a `Value::DictEntry`.
pub fn dict_entry(buf: &BytesMut, offset: &mut usize, is_le: bool,
                  array_recursion: u8, struct_recursion: u8,
                  signature_key: &str, signature_value: &str)
    -> Result<Value, DecodeError> {
    let mut v = Value::decode(buf, offset, is_le, array_recursion,
                              struct_recursion, signature_key)?;
    let k = if v.len() == 1 {
        v.pop().unwrap()
    } else {
        return Err(DecodeError::ArrayVecLen)
    };

    let mut v = Value::decode(buf, offset, is_le, array_recursion,
                              struct_recursion, signature_value)?;
    let v = if v.len() == 1 {
        v.pop().unwrap()
    } else {
        return Err(DecodeError::ArrayVecLen)
    };


    Ok(Value::DictEntry(Box::new((k, v))))
}

impl From<FromUtf8Error> for DecodeError {
    fn from(from_utf8_error: FromUtf8Error) -> Self {
        DecodeError::Utf8Error(from_utf8_error)
    }
}
