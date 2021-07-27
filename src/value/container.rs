use crate::value::{Type, TypeError, Value};
use std::convert::{AsRef, TryFrom};
use thiserror::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Array {
    pub(crate) type_: Type,
    pub(crate) array: Vec<Value>,
}

#[derive(Debug, PartialEq, Error)]
pub enum ArrayError {
    #[error("The type of an array element is different: expected '{0}' got '{1}'")]
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
        let array = Array { type_, array };
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

impl From<Array> for Vec<Value> {
    fn from(array: Array) -> Self {
        array.array
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Struct(pub(crate) Vec<Value>);

#[derive(Debug, PartialEq, Error)]
pub enum StructError {
    #[error("Strcut cannot be empty")]
    Empty,
}

impl AsRef<[Value]> for Struct {
    fn as_ref(&self) -> &[Value] {
        self.0.as_ref()
    }
}

impl TryFrom<Vec<Value>> for Struct {
    type Error = StructError;

    fn try_from(values: Vec<Value>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            Err(StructError::Empty)
        } else {
            Ok(Struct(values))
        }
    }
}

impl From<Struct> for Vec<Value> {
    fn from(struct_: Struct) -> Self {
        struct_.0
    }
}
