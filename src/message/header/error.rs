use thiserror::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Error)]
pub enum MessageHeaderError {
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
