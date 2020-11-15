#[macro_use(lazy_static)]
extern crate lazy_static;
#[macro_use(FromPrimitive, ToPrimitive)]
extern crate num_derive;
#[macro_use(bitflags)]
extern crate bitflags;

mod decoder;
mod encoder;
mod error;
mod message;
mod value;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use error::{DecodeError, DecodeResult, EncodeError, EncodeResult};
pub use message::{
    Message, MessageFlags, MessageHeader, MessageHeaderError, MessageHeaderField, MessageType,
};
pub use value::{
    Bus, BusError, Error, ErrorError, Interface, InterfaceError, Member, MemberError, ObjectPath,
    ObjectPathError, Value, BUS_REGEX, ERROR_REGEX, INTERFACE_REGEX, MAXIMUM_NAME_LENGTH,
    MEMBER_REGEX, OBJECT_PATH_ELEMENT_REGEX, OBJECT_PATH_REGEX,
};
