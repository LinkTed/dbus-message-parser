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

mod decoder;
pub use decoder::Decoder;

mod encoder;
pub use encoder::Encoder;

mod error;
pub use error::{DecodeError, DecodeResult, EncodeError, EncodeResult};

mod header;
pub use header::Header;

mod message;
pub use message::{Message, MessageFlags, MessageHeader, MessageType};

mod value;
pub use value::Value;
