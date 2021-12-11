mod flags;
mod header;
mod message_struct;
mod types;

pub use flags::MessageFlags;
pub use header::{
    Header as MessageHeader, HeaderError as MessageHeaderError,
    HeaderFields as MessageHeaderFields, HeaderFieldsError as MessageHeaderFieldsError,
};
pub use message_struct::Message;
pub use types::MessageType;

/// The maximum length of a message. 128 MiB
pub const MAXIMUM_MESSAGE_LENGTH: usize = 134217728;
