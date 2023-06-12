use crate::std::fmt;

/// Settings for where to store bezel configuration.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BezelConfigStorage {
    /// Settings are stored in RAM, returns to original setting on reset.
    Ram = 0x00,
    /// Settings stored in EEPROM, persists after reset.
    Eeprom = 0x01,
}

impl From<u8> for BezelConfigStorage {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Self::Ram,
            0x01 => Self::Eeprom,
            _ => Self::Ram,
        }
    }
}

impl From<BezelConfigStorage> for u8 {
    fn from(val: BezelConfigStorage) -> Self {
        val as u8
    }
}

impl From<&BezelConfigStorage> for u8 {
    fn from(val: &BezelConfigStorage) -> Self {
        (*val).into()
    }
}

impl From<BezelConfigStorage> for &'static str {
    fn from(val: BezelConfigStorage) -> Self {
        match val {
            BezelConfigStorage::Ram => "RAM",
            BezelConfigStorage::Eeprom => "EEPROM",
        }
    }
}

impl From<&BezelConfigStorage> for &'static str {
    fn from(val: &BezelConfigStorage) -> Self {
        (*val).into()
    }
}

impl fmt::Display for BezelConfigStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
