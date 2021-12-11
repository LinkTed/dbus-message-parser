use crate::value::{
    Bus, BusError, Error, ErrorError, Interface, InterfaceError, Member, MemberError, ObjectPath,
    Struct, Type, Value,
};
use std::convert::TryFrom;
use thiserror::Error as ThisError;

macro_rules! try_set_field {
    ($function:ident, $field:ident, $variant:ident, $multiple_error:ident, $error:ident $(,$convert:expr)?) => {
        fn $function(&mut self, v: Value) -> Result<(), FieldsError> {
            if self.$field.is_some() {
                return Err(FieldsError::$multiple_error(v));
            }

            match v {
                Value::$variant(o) => {
                    $(let o = $convert(o)?;)?
                    self.$field = Some(o);
                    Ok(())
                }
                v => Err(FieldsError::$error(v)),
            }
        }
    };
}

macro_rules! add_to_vec {
    ($vec:ident, $fields:ident, $field:ident, $number:literal, $variant:ident $(,$convert:ident)?) => {
        if let Some(v) = $fields.$field {
            $(let v = v.$convert();)?
            $vec.push(Value::Struct(Struct(vec![
                Value::Byte($number),
                Value::Variant(Box::new(Value::$variant(v))),
            ])));
        }
    };
}

#[derive(Debug, PartialEq, ThisError)]
pub enum FieldsError {
    #[error("Value is not a Struct: {0:?}")]
    Struct(Value),
    #[error("Struct does not contain exactly two values: {0}")]
    Length(usize),
    #[error("Second value is not a Variant: {0:?}")]
    Variant(Value),
    #[error("First value is not a Byte: {0:?}")]
    Byte(Value),
    #[error("Variant does not contain a ObjectPath: {0:?}")]
    Path(Value),
    #[error("Variant does not contain a String: {0:?}")]
    Interface(Value),
    #[error("String could not be converted to an Interface: {0}")]
    InterfaceError(#[from] InterfaceError),
    #[error("Variant does not contain a String: {0:?}")]
    Member(Value),
    #[error("String could not be converted to an Member: {0}")]
    MemberError(#[from] MemberError),
    #[error("Variant does not contain a String: {0:?}")]
    ErrorName(Value),
    #[error("String could not be converted to an ErrorName: {0}")]
    ErrorError(#[from] ErrorError),
    #[error("Variant does not contain a Uint32: {0:?}")]
    ReplySerial(Value),
    #[error("")]
    BusError(#[from] BusError),
    #[error("Variant does not contain a String: {0:?}")]
    Destination(Value),
    #[error("Variant does not contain a String: {0:?}")]
    Sender(Value),
    #[error("Variant does not contain a Signature: {0:?}")]
    Signature(Value),
    #[cfg(target_family = "unix")]
    #[error("Variant does not contain a Uint32: {0:?}")]
    UnixFDs(Value),
    #[error("The byte does not has a valid number: {0}")]
    InvalidNumber(u8),
    #[error("The path is defined mutlple times: {0:?}")]
    MultiplePath(Value),
    #[error("The interface is defined mutlple times: {0:?}")]
    MultipleInterface(Value),
    #[error("The member is defined mutlple times: {0:?}")]
    MultipleMember(Value),
    #[error("The error name is defined mutlple times: {0:?}")]
    MultipleErrorName(Value),
    #[error("The reply serial is defined mutlple times: {0:?}")]
    MultipleReplySerial(Value),
    #[error("The destination is defined mutlple times: {0:?}")]
    MultipleDestination(Value),
    #[error("The destination is defined mutlple times: {0:?}")]
    MultipleSender(Value),
    #[error("The signature is defined mutlple times: {0:?}")]
    MultipleSignature(Value),
    #[cfg(target_family = "unix")]
    #[error("The unix fds is defined mutlple times: {0:?}")]
    MultipleUnixFDs(Value),
}

/// An struct representing the [header fields].
///
/// [header fields]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct Fields {
    pub path: Option<ObjectPath>,
    pub interface: Option<Interface>,
    pub member: Option<Member>,
    pub error_name: Option<Error>,
    pub reply_serial: Option<u32>,
    pub destination: Option<Bus>,
    pub sender: Option<Bus>,
    pub signature: Option<Vec<Type>>,
    #[cfg(target_family = "unix")]
    pub unix_fds: Option<u32>,
}

impl Fields {
    try_set_field!(try_set_path, path, ObjectPath, MultiplePath, Path);
    try_set_field!(
        try_set_interface,
        interface,
        String,
        MultipleInterface,
        Interface,
        Interface::try_from
    );
    try_set_field!(
        try_set_member,
        member,
        String,
        MultipleMember,
        Member,
        Member::try_from
    );
    try_set_field!(
        try_set_error_name,
        error_name,
        String,
        MultipleErrorName,
        ErrorName,
        Error::try_from
    );
    try_set_field!(
        try_set_reply_serial,
        reply_serial,
        Uint32,
        MultipleReplySerial,
        ReplySerial
    );
    try_set_field!(
        try_set_destination,
        destination,
        String,
        MultipleDestination,
        Destination,
        Bus::try_from
    );
    try_set_field!(
        try_set_sender,
        sender,
        String,
        MultipleSender,
        Sender,
        Bus::try_from
    );
    try_set_field!(
        try_set_signature,
        signature,
        Signature,
        MultipleSignature,
        Signature
    );
    #[cfg(target_family = "unix")]
    try_set_field!(try_set_unix_fds, unix_fds, Uint32, MultipleUnixFDs, UnixFDs);

    fn try_set_field(&mut self, b: u8, v: Value) -> Result<(), FieldsError> {
        match b {
            1 => self.try_set_path(v),
            2 => self.try_set_interface(v),
            3 => self.try_set_member(v),
            4 => self.try_set_error_name(v),
            5 => self.try_set_reply_serial(v),
            6 => self.try_set_destination(v),
            7 => self.try_set_sender(v),
            8 => self.try_set_signature(v),
            #[cfg(target_family = "unix")]
            9 => self.try_set_unix_fds(v),
            // Invalid number.
            b => Err(FieldsError::InvalidNumber(b)),
        }
    }
}

fn unwrap_value(value: Value) -> Result<(u8, Value), FieldsError> {
    // The outer `Value` has to be a struct.
    let mut values: Vec<Value> = match value {
        Value::Struct(struct_) => struct_.into(),
        v => return Err(FieldsError::Struct(v)),
    };
    // The length of the struct have to be 2
    let values_len = values.len();
    if values_len != 2 {
        return Err(FieldsError::Length(values_len));
    }
    // Check if the second is a Variant and unwrap the value.
    let v = match values.pop().unwrap() {
        Value::Variant(v) => *v,
        v => return Err(FieldsError::Variant(v)),
    };
    // Check if the first is a byte
    let b = match values.pop().unwrap() {
        Value::Byte(b) => b,
        v => return Err(FieldsError::Byte(v)),
    };

    Ok((b, v))
}

impl TryFrom<Vec<Value>> for Fields {
    type Error = FieldsError;

    fn try_from(values: Vec<Value>) -> Result<Self, Self::Error> {
        let mut fields = Fields::default();

        for value in values {
            let (b, v) = unwrap_value(value)?;
            fields.try_set_field(b, v)?;
        }

        Ok(fields)
    }
}

impl From<Fields> for Vec<Value> {
    fn from(fields: Fields) -> Self {
        let mut values = Vec::new();

        add_to_vec!(values, fields, path, 1, ObjectPath);
        add_to_vec!(values, fields, interface, 2, String, to_string);
        add_to_vec!(values, fields, member, 3, String, to_string);
        add_to_vec!(values, fields, error_name, 4, String, to_string);
        add_to_vec!(values, fields, reply_serial, 5, Uint32);
        add_to_vec!(values, fields, destination, 6, String, to_string);
        add_to_vec!(values, fields, sender, 7, String, to_string);
        add_to_vec!(values, fields, signature, 8, Signature);
        #[cfg(target_family = "unix")]
        add_to_vec!(values, fields, unix_fds, 9, Uint32);
        values
    }
}
