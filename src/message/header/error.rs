use thiserror::Error as ThisError;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, ThisError)]
pub enum Error {
    #[error("Path is missing")]
    MissingPath,
    #[error("Interface is missing")]
    MissingInterface,
    #[error("Member is missing")]
    MissingMember,
    #[error("ErrorName is missing")]
    MissingErrorName,
    #[error("ReplySerial is missing")]
    MissingReplySerial,
}
