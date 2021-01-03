mod bus;
mod error;
mod interface;
mod member;
mod object_path;
mod signature;
mod value_enum;

pub use bus::{Bus, BusError, BUS_REGEX};
pub use error::{Error, ErrorError, ERROR_REGEX};
pub use interface::{Interface, InterfaceError, INTERFACE_REGEX};
pub use member::{Member, MemberError, MEMBER_REGEX};
pub use object_path::{ObjectPath, ObjectPathError, OBJECT_PATH_ELEMENT_REGEX, OBJECT_PATH_REGEX};
pub use signature::{Signature, SignatureError, SignatureIter, MAXIMUM_SIGNATURE_LENGTH};
pub use value_enum::{Type, Value};

pub const MAXIMUM_NAME_LENGTH: usize = 255;
pub const MAXIMUM_ARRAY_LENGTH: usize = 67108864;

pub const MAXIMUM_ARRAY_DEPTH: u8 = 32;
pub const MAXIMUM_STRUCT_DEPTH: u8 = 32;
pub const MAXIMUM_DICT_DEPTH: u8 = 32;
