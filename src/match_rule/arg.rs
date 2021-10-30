use super::{escape::escape, MatchRuleError};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

pub const MAXIMUM_ARG_INDEX: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub struct Arg(usize, String);

impl Arg {
    pub fn get_key(&self) -> String {
        format!("arg{}", self.get_index())
    }

    pub fn get_index(&self) -> usize {
        self.0
    }

    pub fn get_value(&self) -> &str {
        self.1.as_str()
    }
}

impl TryFrom<(usize, String)> for Arg {
    type Error = MatchRuleError;

    fn try_from(arg: (usize, String)) -> Result<Self, Self::Error> {
        if MAXIMUM_ARG_INDEX <= arg.0 {
            return Err(MatchRuleError::ArgIndexTooBig(arg.0));
        }
        Ok(Arg(arg.0, arg.1))
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", escape(self.1.as_str()))
    }
}
