mod arg;
mod arg_path;
mod decode;
mod encode;
mod escape;
mod matching;
mod split;
mod unescape;

use crate::{
    message::MessageType,
    value::{
        Bus, BusError, Interface, InterfaceError, Member, MemberError, ObjectPath, ObjectPathError,
        UniqueConnectionName, UniqueConnectionNameError,
    },
};
pub use arg::{Arg, MAXIMUM_ARG_INDEX};
pub use arg_path::ArgPath;
use std::num::ParseIntError;
use thiserror::Error;

/// This represents an [Match Rule].
///
/// [Match Rule]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-routing-match-rules
#[derive(Debug, PartialEq, Eq)]
pub enum MatchRule {
    Type(MessageType),
    Sender(Bus),
    Interface(Interface),
    Member(Member),
    Path(ObjectPath),
    PathNamespace(ObjectPath),
    Destination(UniqueConnectionName),
    Arg(Arg),
    ArgPath(ArgPath),
    Arg0Namespace(Interface),
    Eavesdrop(bool),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum MatchRuleError {
    #[error("Key contains invalid character: {0}")]
    KeyInvalidChar(char),
    #[error("Key is emtpy")]
    KeyEmpty,
    #[error("Key is unknown")]
    KeyUnknown,
    #[error("The equal character is missing")]
    MissingEqual,
    #[error("The closing quote is missing")]
    ValueClosingQuote,
    #[error("Could not decode sender: {0}")]
    SenderError(#[from] BusError),
    #[error("Could not decode type")]
    TypeUnknown,
    #[error("Could not decode interface: {0}")]
    InterfaceError(InterfaceError),
    #[error("Could not decode member: {0}")]
    MemberError(#[from] MemberError),
    #[error("Could not decode path: {0}")]
    PathError(ObjectPathError),
    #[error("Could not decode path namespace: {0}")]
    PathErrorNamespace(ObjectPathError),
    #[error("Could not decode destination: {0}")]
    DestinationError(#[from] UniqueConnectionNameError),
    #[error("The index of the argument is too big: {0} < {MAXIMUM_ARG_INDEX}")]
    ArgIndexTooBig(usize),
    #[error("Could not decode arg path: {0}")]
    ArgPathError(ObjectPathError),
    #[error("Could not parse index: {0}")]
    ArgIndexError(#[from] ParseIntError),
    #[error("Could not decode arg0 namespace: {0}")]
    Arg0NamespaceError(InterfaceError),
    #[error("Could not decode eavesdrop")]
    EavesdropUnknown,
}
