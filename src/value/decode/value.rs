use crate::{DecodeError, DecodeResult, Decoder, Value};
use std::ops::Deref;

/// Get the next signature from a `&str`.
fn get_next_signature<'a>(
    signature: &'a str,
    signature_offset: &mut usize,
) -> DecodeResult<&'a str> {
    let start = *signature_offset;

    loop {
        let mut end = *signature_offset + 1;

        match signature.get(*signature_offset..end) {
            Some(s) => match s {
                "y" | "b" | "n" | "q" | "i" | "u" | "x" | "t" | "d" | "s" | "o" | "g" | "v" => {
                    return Ok(&signature[start..end]);
                }
                #[cfg(target_family = "unix")]
                "h" => {
                    return Ok(&signature[start..end]);
                }
                "a" => {}
                "(" => {
                    let mut parentheses_depth: usize = 0;

                    loop {
                        *signature_offset = end;
                        end = *signature_offset + 1;

                        if let Some(s) = signature.get(*signature_offset..end) {
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

                        if let Some(s) = signature.get(*signature_offset..end) {
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
                _ => return Err(DecodeError::Signature),
            },
            None => return Err(DecodeError::Signature),
        }

        *signature_offset = end;
    }
}

impl<'a, T> Decoder<'a, T>
where
    T: Deref<Target = [u8]>,
{
    /// Decode a byte array to a `Value` object.
    pub(crate) fn v(
        &mut self,
        is_le: bool,
        array_recursion: u8,
        struct_recursion: u8,
        sig: &str,
    ) -> DecodeResult<Vec<Value>> {
        // Check if the signature is not too long.
        if 256 <= sig.len() {
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
        while let Some(s) = sig.get(signature_offset..end) {
            let v = match s {
                "y" => self.byte()?,
                "b" => {
                    self.algin(4)?;
                    self.boolean(is_le)?
                }
                "n" => {
                    self.algin(2)?;
                    self.int_16(is_le)?
                }
                "q" => {
                    self.algin(2)?;
                    self.uint_16(is_le)?
                }
                "i" => {
                    self.algin(4)?;
                    self.int_32(is_le)?
                }
                "u" => {
                    self.algin(4)?;
                    self.uint_32(is_le)?
                }
                "x" => {
                    self.algin(8)?;
                    self.int_64(is_le)?
                }
                "t" => {
                    self.algin(8)?;
                    self.uint_64(is_le)?
                }
                "d" => {
                    self.algin(8)?;
                    self.double(is_le)?
                }
                "s" => {
                    self.algin(4)?;
                    self.string(is_le)?
                }
                "o" => {
                    self.algin(4)?;
                    self.object_path(is_le)?
                }
                "g" => self.signature()?,
                #[cfg(target_family = "unix")]
                "h" => {
                    self.algin(4)?;
                    self.unix_fd(is_le)?
                }
                "a" => {
                    self.algin(4)?;
                    signature_offset += 1;
                    let signature = get_next_signature(sig, &mut signature_offset)?;
                    self.array(is_le, array_recursion, struct_recursion, signature)?
                }
                "(" => {
                    self.algin(8)?;
                    // We have to the find inner signature of the struct by
                    // searching the closing parenthesis.
                    signature_offset += 1;
                    let mut bracket_depth = 0;
                    let start = signature_offset;
                    loop {
                        let end = signature_offset + 1;
                        // Check if we reach the end of the signature.
                        // So we did not find the closing parenthesis.
                        if sig.len() <= signature_offset {
                            return Err(DecodeError::Signature);
                        }
                        // Get the next character.
                        if let Some(s) = sig.get(signature_offset..end) {
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
                                    break self.decode_struct(
                                        is_le,
                                        array_recursion,
                                        struct_recursion,
                                        &sig[start..signature_offset],
                                    )?;
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
                    self.algin(8)?;
                    // We have to parse the inner signature of the dict.
                    // The dict has to contain exactly two signature.
                    // Parse the first signature, it is the signature of the
                    // key.
                    signature_offset += 1;
                    let signature_key = get_next_signature(sig, &mut signature_offset)?;
                    // Parse the second signature, it is the signature of the
                    // value
                    signature_offset += 1;
                    let signature_value = get_next_signature(sig, &mut signature_offset)?;

                    signature_offset += 1;
                    end = signature_offset + 1;
                    // Get the next character.
                    if let Some(s) = sig.get(signature_offset..end) {
                        // It has to be a closing curly bracket.
                        if s == "}" {
                            self.dict_entry(
                                is_le,
                                array_recursion,
                                struct_recursion,
                                signature_key,
                                signature_value,
                            )?
                        } else {
                            return Err(DecodeError::Signature);
                        }
                    } else {
                        return Err(DecodeError::Signature);
                    }
                }
                "v" => self.variant(is_le)?,
                _ => return Err(DecodeError::Signature),
            };

            signature_offset += 1;
            end = signature_offset + 1;

            r.push(v);
        }

        Ok(r)
    }
}
