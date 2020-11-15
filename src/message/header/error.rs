#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum MessageHeaderError {
    MissingPath,
    MissingInterface,
    MissingMember,
    MissingErrorName,
    MissingReplySerial,
}
