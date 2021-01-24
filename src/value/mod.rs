mod bus;
mod container;
mod error;
mod interface;
mod member;
mod object_path;
mod type_enum;
mod value_enum;

pub use bus::{Bus, BusError};
pub use container::{Array, ArrayError, Struct, StructError};
pub use error::{Error, ErrorError};
pub use interface::{Interface, InterfaceError};
pub use member::{Member, MemberError};
pub use object_path::{ObjectPath, ObjectPathError};
pub use type_enum::{Type, TypeError, MAXIMUM_SIGNATURE_LENGTH};
pub use value_enum::Value;

pub const MAXIMUM_NAME_LENGTH: usize = 255;
pub const MAXIMUM_ARRAY_LENGTH: usize = 67108864;

pub const MAXIMUM_ARRAY_DEPTH: u8 = 32;
pub const MAXIMUM_STRUCT_DEPTH: u8 = 32;
pub const MAXIMUM_DICT_DEPTH: u8 = 32;
