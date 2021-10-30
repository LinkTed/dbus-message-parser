use crate::value::ObjectPath;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, Eq)]
pub struct ArgPath(usize, ObjectPath);

impl ArgPath {
    pub fn get_key(&self) -> String {
        format!("arg{}path", self.get_index())
    }

    pub fn get_index(&self) -> usize {
        self.0
    }

    pub fn get_value(&self) -> &ObjectPath {
        &self.1
    }
}

impl From<(usize, ObjectPath)> for ArgPath {
    fn from(arg: (usize, ObjectPath)) -> Self {
        ArgPath(arg.0, arg.1)
    }
}

impl Display for ArgPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", &self.1)
    }
}
