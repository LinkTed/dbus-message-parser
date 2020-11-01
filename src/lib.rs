#[macro_use(lazy_static)]
extern crate lazy_static;
#[macro_use(FromPrimitive, ToPrimitive)]
extern crate num_derive;
#[macro_use(bitflags)]
extern crate bitflags;

mod decoder;
mod encoder;
mod error;
mod header;
mod message;
mod value;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use error::{DecodeError, DecodeResult, EncodeError, EncodeResult};
pub use header::Header;
pub use message::{Message, MessageFlags, MessageHeader, MessageType};
pub use value::{
    Bus, BusError, Interface, InterfaceError, Member, MemberError, ObjectPath, ObjectPathError,
    Value, BUS_REGEX, INTERFACE_REGEX, MAXIMUM_NAME_LENGTH, MEMBER_REGEX,
    OBJECT_PATH_ELEMENT_REGEX, OBJECT_PATH_REGEX,
};
