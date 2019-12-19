#[macro_use(lazy_static)]
extern crate lazy_static;
#[macro_use(FromPrimitive, ToPrimitive)]
extern crate num_derive;
#[macro_use(bitflags)]
extern crate bitflags;

use regex::Regex;


lazy_static! {
    /// The regular expression for a valid [object path].
    ///
    /// [object path]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-marshaling-object-path
    static ref OBJECT_PATH_REGEX: Regex = Regex::new("/([A-Za-z0-9_]+(/[A-Za-z0-9_]+)*)?").unwrap();
}

mod value;
mod header;
mod message;

pub use value::{Value, DecodeError, EncodeError};
pub use message::{Message, MessageType, MessageHeader};
