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
    pub static ref OBJECT_PATH_REGEX: Regex = Regex::new(r"^/([A-Za-z0-9_]+(/[A-Za-z0-9_]+)*)?$").unwrap();

    /// The regular expression for a valid [interface name].
    ///
    /// [interface name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
    pub static ref INTERFACE_REGEX: Regex = Regex::new("^[A-Za-z0-9_]+(.[A-Za-z0-9_]+)+$").unwrap();

    /// The regular expression for a valid [member name].
    ///
    /// [member name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
    pub static ref MEMBER_REGEX: Regex = Regex::new("^[A-Za-z_][A-Za-z0-9_]*$").unwrap();

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
pub use value::Value;
