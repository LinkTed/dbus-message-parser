#[macro_use(lazy_static)]
extern crate lazy_static;
#[macro_use(FromPrimitive, ToPrimitive)]
extern crate num_derive;
#[macro_use(bitflags)]
extern crate bitflags;

use regex::Regex;

lazy_static! {
    /// The regular expression for a valid [bus name].
    ///
    /// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
    pub static ref BUS_NAMES: Regex = Regex::new("^:?[A-Za-z0-9_-]+(.[A-Za-z0-9_-]+)+$").unwrap();
}

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
    Interface, InterfaceError, Member, MemberError, ObjectPath, ObjectPathError, Value,
    INTERFACE_REGEX, MAXIMUM_NAME_LENGTH, MEMBER_REGEX, OBJECT_PATH_ELEMENT_REGEX,
    OBJECT_PATH_REGEX,
};
