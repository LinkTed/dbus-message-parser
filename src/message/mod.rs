mod flags;
mod header;
mod message_struct;
mod types;

pub use flags::MessageFlags;
pub use header::{MessageHeader, MessageHeaderError, MessageHeaderField, MessageHeaderFieldError};
pub use message_struct::Message;
pub use types::MessageType;

/// The maximum length of a message. 128 MiB
pub const MAXIMUM_MESSAGE_LENGTH: usize = 134217728;
