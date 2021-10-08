use std::cmp::{Eq, PartialEq};
use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

mod unique_connection_name;
mod well_known_bus_name;

pub use unique_connection_name::{UniqueConnectionName, UniqueConnectionNameError};
pub use well_known_bus_name::{WellKnownBusName, WellKnownBusNameError};

/// This represents a [bus name].
///
/// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum Bus {
    UniqueConnectionName(UniqueConnectionName),
    WellKnownBusName(WellKnownBusName),
}

/// An enum representing all errors, which can occur during the handling of a [`Bus`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum BusError {
    #[error("Could not parse unique connection name: {0}")]
    UniqueConnectionNameError(#[from] UniqueConnectionNameError),
    #[error("Could not parse well-known bus name: {0}")]
    WellKnownBusNameError(#[from] WellKnownBusNameError),
}

impl From<Bus> for String {
    fn from(bus: Bus) -> Self {
        match bus {
            Bus::UniqueConnectionName(unique_connection_name) => unique_connection_name.into(),
            Bus::WellKnownBusName(well_known_bus_name) => well_known_bus_name.into(),
        }
    }
}

impl TryFrom<String> for Bus {
    type Error = BusError;

    fn try_from(bus: String) -> Result<Self, Self::Error> {
        if bus.starts_with(':') {
            let unique_connection_name = UniqueConnectionName::try_from(bus)?;
            Ok(Bus::UniqueConnectionName(unique_connection_name))
        } else {
            let well_known_bus_name = WellKnownBusName::try_from(bus)?;
            Ok(Bus::WellKnownBusName(well_known_bus_name))
        }
    }
}

impl TryFrom<&str> for Bus {
    type Error = BusError;

    fn try_from(bus: &str) -> Result<Self, Self::Error> {
        if bus.starts_with(':') {
            let unique_connection_name = UniqueConnectionName::try_from(bus)?;
            Ok(Bus::UniqueConnectionName(unique_connection_name))
        } else {
            let well_known_bus_name = WellKnownBusName::try_from(bus)?;
            Ok(Bus::WellKnownBusName(well_known_bus_name))
        }
    }
}

impl TryFrom<&[u8]> for Bus {
    type Error = BusError;

    fn try_from(bus: &[u8]) -> Result<Self, Self::Error> {
        if bus.starts_with(&[b':'][..]) {
            let unique_connection_name = UniqueConnectionName::try_from(bus)?;
            Ok(Bus::UniqueConnectionName(unique_connection_name))
        } else {
            let well_known_bus_name = WellKnownBusName::try_from(bus)?;
            Ok(Bus::WellKnownBusName(well_known_bus_name))
        }
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Bus::UniqueConnectionName(unique_connection_name) => {
                write!(f, "{}", unique_connection_name)
            }
            Bus::WellKnownBusName(well_known_bus_name) => {
                write!(f, "{}", well_known_bus_name)
            }
        }
    }
}

impl AsRef<str> for Bus {
    fn as_ref(&self) -> &str {
        match self {
            Bus::UniqueConnectionName(unique_connection_name) => unique_connection_name.as_ref(),
            Bus::WellKnownBusName(well_known_bus_name) => well_known_bus_name.as_ref(),
        }
    }
}

impl PartialEq<str> for Bus {
    fn eq(&self, other: &str) -> bool {
        if other.starts_with(':') {
            match self {
                Bus::UniqueConnectionName(unique_connection_name) => {
                    unique_connection_name.eq(other)
                }
                Bus::WellKnownBusName(_) => false,
            }
        } else {
            match self {
                Bus::UniqueConnectionName(_) => false,
                Bus::WellKnownBusName(well_known_bus_name) => well_known_bus_name.eq(other),
            }
        }
    }
}
