use crate::value::{Type, TypeError, Value};
use std::convert::AsRef;
use thiserror::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Array {
    pub(crate) type_: Type,
    pub(crate) array: Vec<Value>,
}

#[derive(Debug, PartialEq, Error)]
pub enum ArrayError {
    #[error("The type_ of an array element is different: expected '{0}' got '{1}'")]
    TypeMismatch(Type, Type),
    #[error("Coult not get type of element: {0}")]
    TypeError(#[from] TypeError),
}

impl Array {
    pub fn new(array: Vec<Value>, type_: Type) -> Result<Array, ArrayError> {
        for v in &array {
            let s = v.get_type()?;
            if s != type_ {
                return Err(ArrayError::TypeMismatch(type_, s));
            }
        }
        let array = Array { array, type_ };
        Ok(array)
    }

    #[inline]
    pub const fn get_type(&self) -> &Type {
        &self.type_
    }
}

impl AsRef<[Value]> for Array {
    fn as_ref(&self) -> &[Value] {
        self.array.as_ref()
    }
}
