use bytes::BytesMut;
use crate::OBJECT_PATH_REGEX;

mod encode;

pub use encode::{encode_algin, EncodeError};

mod decode;

pub use decode::{decode_algin, DecodeError};


/// Get the next signature from a `&str`.
fn get_next_signature<'a>(signature: &'a str, signature_offset: &mut usize)
    -> Result<&'a str, DecodeError> {
    let start = *signature_offset;

    loop {
        let mut end = *signature_offset + 1;

        match signature.get(*signature_offset..end) {
            Some(s) => {
                match s {
                    "y" | "b" | "n" | "q" | "i" | "u" | "x" | "t" | "d" | "s" |
                    "o" | "g" | "v" => {
                        return Ok(&signature[start..end]);
                    }
                    "a" => {}
                    "(" => {
                        let mut parentheses_depth: usize = 0;

                        loop {
                            *signature_offset = end;
                            end = *signature_offset + 1;

                            if let Some(s) = signature.get(
                                *signature_offset..end) {
                                if s == ")" {
                                    if parentheses_depth == 0 {
                                        return Ok(&signature[start..end]);
                                    } else {
                                        parentheses_depth -= 1;
                                    }
                                } else if s == "(" {
                                    parentheses_depth += 1;
                                }
                            } else {
                                return Err(DecodeError::Signature);
                            }
                        }
                    }
                    "{" => {
                        let mut bracket_depth: usize = 0;

                        loop {
                            *signature_offset = end;
                            end = *signature_offset + 1;

                            if let Some(s) = signature.get(
                                *signature_offset..end) {
                                if s == "}" {
                                    if bracket_depth == 0 {
                                        return Ok(&signature[start..end]);
                                    } else {
                                        bracket_depth -= 1;
                                    }
                                } else if s == "{" {
                                    bracket_depth += 1;
                                }
                            } else {
                                return Err(DecodeError::Signature);
                            }
                        }
                    }
                    _ => return Err(DecodeError::Signature)
                }
            }
            None => return Err(DecodeError::Signature)
        }

        *signature_offset = end;
    }
}

/// An enum representing a [DBus value].
///
/// [DBus value]: https://dbus.freedesktop.org/doc/dbus-specification.html#type-system
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Byte(u8),
    Boolean(bool),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Double(f64),
    String(String),
    ObjectPath(String),
    Signature(String),
    Array(Vec<Value>, String),
    Struct(Vec<Value>),
    DictEntry(Box<(Value, Value)>),
    Variant(Vec<Value>),
    UnixFD(u32),
}

impl Value {
    /// Decode a byte array to a `Value` object.
    pub fn decode(buf: &BytesMut, offset: &mut usize, is_le: bool,
                  array_recursion: u8, struct_recursion: u8, signature: &str)
        -> Result<Vec<Value>, DecodeError> {
        // Check if the signature is not too long.
        if 256 <= signature.len() {
            return Err(DecodeError::SignatureTooBig);
        }
        // Check if the array recursion is not exceeded.
        if 32 <= array_recursion {
            return Err(DecodeError::ArrayRecursion);
        }
        // Check if the struct recursion is not exceeded.
        if 32 <= struct_recursion {
            return Err(DecodeError::StructRecursion);
        }

        let mut signature_offset: usize = 0;
        let mut r = Vec::new();
        let mut end = signature_offset + 1;
        // Decode the value according to the signature.
        while let Some(s) = signature.get(signature_offset..end) {
            let v = match s {
                "y" => decode::byte(buf, offset),
                "b" => {
                    decode::decode_algin(buf, offset, 4)?;
                    decode::boolean(buf, offset, is_le)
                }
                "n" => {
                    decode::decode_algin(buf, offset, 2)?;
                    decode::int16(buf, offset, is_le)
                }
                "q" => {
                    decode::decode_algin(buf, offset, 2)?;
                    decode::uint16(buf, offset, is_le)
                }
                "i" => {
                    decode::decode_algin(buf, offset, 4)?;
                    decode::int32(buf, offset, is_le)
                }
                "u" => {
                    decode::decode_algin(buf, offset, 4)?;
                    decode::uint32(buf, offset, is_le)
                }
                "x" => {
                    decode::decode_algin(buf, offset, 8)?;
                    decode::int64(buf, offset, is_le)
                }
                "t" => {
                    decode::decode_algin(buf, offset, 8)?;
                    decode::uint64(buf, offset, is_le)
                }
                "d" => {
                    decode::decode_algin(buf, offset, 8)?;
                    decode::double(buf, offset, is_le)
                }
                "s" => {
                    decode::decode_algin(buf, offset, 4)?;
                    decode::string(buf, offset, is_le)
                }
                "o" => {
                    decode::decode_algin(buf, offset, 4)?;
                    decode::path(buf, offset, is_le)
                }
                "g" => decode::signature(buf, offset),
                "a" => {
                    decode::decode_algin(buf, offset, 4)?;
                    signature_offset += 1;
                    decode::array(buf, offset, is_le, array_recursion,
                                  struct_recursion,
                                  get_next_signature(signature,
                                                     &mut signature_offset)?)
                }
                "(" => {
                    decode::decode_algin(buf, offset, 8)?;
                    // We have to the find inner signature of the struct by
                    // searching the closing parenthesis.
                    signature_offset += 1;
                    let mut bracket_depth = 0;
                    let start = signature_offset;
                    loop {
                        let end = signature_offset + 1;
                        // Check if we reach the end of the signature.
                        // So we did not find the closing parenthesis.
                        if signature.len() <= signature_offset {
                            return Err(DecodeError::Signature);
                        }
                        // Get the next character.
                        if let Some(s) = signature.get(signature_offset..end) {
                            // Check if the character is a closing parenthesis.
                            if s == ")" {
                                // Check if the right closing parenthesis was
                                // found.
                                if bracket_depth == 0 {
                                    // The length of the signature has to be
                                    // at least one.
                                    if signature_offset - start == 0 {
                                        return Err(DecodeError::Signature);
                                    }
                                    // Decode the struct.
                                    break decode::decode_struct(buf, offset,
                                                                is_le,
                                                                array_recursion,
                                                                struct_recursion,
                                                                &signature[start..signature_offset]);
                                } else {
                                    bracket_depth -= 1;
                                }
                            } else if s == "(" {
                                bracket_depth += 1;
                            }
                        } else {
                            return Err(DecodeError::Signature);
                        }

                        signature_offset = end;
                    }
                }
                "{" => {
                    decode::decode_algin(buf, offset, 8)?;
                    // We have to parse the inner signature of the dict.
                    // The dict has to contain exactly two signature.
                    // Parse the first signature, it is the signature of the
                    // key.
                    signature_offset += 1;
                    let signature_key =
                        get_next_signature(signature, &mut signature_offset)?;
                    // Parse the second signature, it is the signature of the
                    // value
                    signature_offset += 1;
                    let signature_value =
                        get_next_signature(signature, &mut signature_offset)?;

                    signature_offset += 1;
                    end = signature_offset + 1;
                    // Get the next character.
                    if let Some(s) = signature.get(signature_offset..end) {
                        // It has to be a closing curly bracket.
                        if s == "}" {
                            decode::dict_entry(buf, offset, is_le,
                                               array_recursion,
                                               struct_recursion,
                                               signature_key,
                                               signature_value)
                        } else {
                            return Err(DecodeError::Signature);
                        }
                    } else {
                        return Err(DecodeError::Signature);
                    }
                }
                "v" => decode::variant(buf, offset, is_le),
                _ => return Err(DecodeError::Signature)
            }?;

            signature_offset += 1;
            end = signature_offset + 1;

            r.push(v);
        }

        Ok(r)
    }

    /// Encode a `Value` object to a byte array.
    pub fn encode(&self, buf: &mut BytesMut, is_le: bool)
                  -> Result<(), EncodeError> {
        match self {
            Value::Byte(b) => encode::byte(buf, *b),
            Value::Boolean(b) => {
                encode::encode_algin(buf, 4);
                encode::boolean(buf, *b, is_le)
            }
            Value::Int16(i) => {
                encode::encode_algin(buf, 2);
                encode::i16(buf, *i, is_le)
            }
            Value::Uint16(u) => {
                encode::encode_algin(buf, 2);
                encode::u16(buf, *u, is_le)
            }
            Value::Int32(i) => {
                encode::encode_algin(buf, 4);
                encode::i32(buf, *i, is_le)
            }
            Value::Uint32(u) => {
                encode::encode_algin(buf, 4);
                encode::u32(buf, *u, is_le)
            }
            Value::Int64(i) => {
                encode::encode_algin(buf, 8);
                encode::i64(buf, *i, is_le)
            }
            Value::Uint64(u) => {
                encode::encode_algin(buf, 8);
                encode::u64(buf, *u, is_le)
            }
            Value::Double(f) => {
                encode::encode_algin(buf, 8);
                encode::f64(buf, *f, is_le)
            }
            Value::ObjectPath(s) => {
                if !OBJECT_PATH_REGEX.is_match(s) {
                    return Err(EncodeError::ObjectPathInvalid(s.clone()));
                }

                encode::encode_algin(buf, 4);
                encode::str(buf, s, is_le)
            }
            Value::String(s) => {
                encode::encode_algin(buf, 4);
                encode::str(buf, s, is_le)
            }
            Value::Signature(s) => encode::sig(buf, s),
            Value::Array(vec, sig) => {
                encode::encode_algin(buf, 4);
                encode::array(buf, vec, sig, is_le)
            }
            Value::Struct(vec) => {
                encode::encode_algin(buf, 8);
                encode::encode_struct(buf, vec, is_le)
            }
            Value::DictEntry(b) => {
                encode::encode_algin(buf, 8);
                encode::dict_entry(buf, b, is_le)
            }
            Value::Variant(vec) => encode::variant(buf, vec, is_le),
            Value::UnixFD(pos) => {
                encode::encode_algin(buf, 4);
                encode::u32(buf, *pos, is_le)
            }
        }
    }

    /// Write the signature of the `Value` object into the `s` argument.
    pub fn get_signature(&self, s: &mut String) {
        match self {
            Value::Byte(_) => s.push('y'),
            Value::Boolean(_) => s.push('b'),
            Value::Int16(_) => s.push('n'),
            Value::Uint16(_) => s.push('q'),
            Value::Int32(_) => s.push('i'),
            Value::Uint32(_) => s.push('u'),
            Value::Int64(_) => s.push('x'),
            Value::Uint64(_) => s.push('t'),
            Value::Double(_) => s.push('d'),
            Value::String(_) => s.push('s'),
            Value::ObjectPath(_) => s.push('o'),
            Value::Signature(_) => s.push('g'),
            Value::Array(_, sig) => {
                s.push('a');
                s.push_str(sig);
            }
            Value::Struct(vec) => {
                s.push('(');
                for v in vec {
                    v.get_signature(s);
                }
                s.push(')');
            }
            Value::DictEntry(b) => {
                s.push('{');
                let (key, value) = &**b;
                key.get_signature(s);
                value.get_signature(s);
                s.push('}');
            }
            Value::Variant(_) => s.push('v'),
            Value::UnixFD(_) => s.push('h')
        }
    }
}
