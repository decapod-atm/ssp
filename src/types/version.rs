//! Version information types.

use crate::std::{self, fmt};

use crate::{impl_default, tuple_struct_ser};

/// Protocol version supported by device firmware
///
/// The highest protocol version a device will support is determined by its firmware.
///
/// | **NV9USB** |
/// |:----------:|
///
/// | Protocol Version | Firmware Version |
/// |:----------------:|:----------------:|
/// | 6                | 3.27             |
/// | 7                | 3.33             |
/// | 8                | -                |
///
/// | **NV11** |
/// |:--------:|
///
/// | Protocol Version | Firmware Version |
/// |:----------------:|:----------------:|
/// | 6                | 3.27             |
/// | 7                | 3.33             |
/// | 8                | -                |
///
/// | **NV200** |
/// |:---------:|
///
/// | Protocol Version | Firmware Version |
/// |:----------------:|:----------------:|
/// | 6                | 4.07             |
/// | 7                | 4.08             |
/// | 8                | 4.09             |
///
/// | **SMART Payout** |
/// |:----------------:|
///
/// | Protocol Version | Firmware Version |
/// |:----------------:|:----------------:|
/// | 6                | 4.07             |
/// | 7                | 4.08             |
/// | 8                | 4.09             |
///
/// | **SMART Hopper** |
/// |:----------------:|
///
/// | Protocol Version | Firmware Version |
/// |:----------------:|:----------------:|
/// | 6                | 6.03             |
/// | 7                | 6.09             |
/// | 8                | -                |
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProtocolVersion {
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
    Eight = 0x08,
    Reserved = 0xff,
}

impl ProtocolVersion {
    /// Creates a new [ProtocolVersion].
    ///
    /// Default value is `0x06`, since this seems to be the base protocol for most devices.
    pub const fn new() -> Self {
        Self::Six
    }

    /// Converts a `u8` into a [ProtocolVersion].
    pub const fn from_u8(val: u8) -> Self {
        match val {
            0x01 => Self::One,
            0x02 => Self::Two,
            0x03 => Self::Three,
            0x04 => Self::Four,
            0x05 => Self::Five,
            0x06 => Self::Six,
            0x07 => Self::Seven,
            0x08 => Self::Eight,
            _ => Self::Reserved,
        }
    }

    /// Converts a [ProtocolVersion] into a `u8`.
    pub const fn to_u8(&self) -> u8 {
        match self {
            ProtocolVersion::One => 0x01,
            ProtocolVersion::Two => 0x02,
            ProtocolVersion::Three => 0x03,
            ProtocolVersion::Four => 0x04,
            ProtocolVersion::Five => 0x05,
            ProtocolVersion::Six => 0x06,
            ProtocolVersion::Seven => 0x07,
            ProtocolVersion::Eight => 0x08,
            ProtocolVersion::Reserved => 0xff,
        }
    }
}

impl From<u8> for ProtocolVersion {
    fn from(val: u8) -> Self {
        Self::from_u8(val)
    }
}

impl From<ProtocolVersion> for u8 {
    fn from(val: ProtocolVersion) -> Self {
        val.to_u8()
    }
}

impl From<&ProtocolVersion> for u8 {
    fn from(val: &ProtocolVersion) -> Self {
        (*val).into()
    }
}

impl From<ProtocolVersion> for &'static str {
    fn from(val: ProtocolVersion) -> Self {
        match val {
            ProtocolVersion::One => "1",
            ProtocolVersion::Two => "2",
            ProtocolVersion::Three => "3",
            ProtocolVersion::Four => "4",
            ProtocolVersion::Five => "5",
            ProtocolVersion::Six => "6",
            ProtocolVersion::Seven => "7",
            ProtocolVersion::Eight => "8",
            ProtocolVersion::Reserved => "255",
        }
    }
}

impl From<&ProtocolVersion> for &'static str {
    fn from(val: &ProtocolVersion) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl_default!(ProtocolVersion);

tuple_struct_ser!(
    FirmwareVersion,
    u32,
    "ASCII-encoded firmware version for the device."
);

impl From<&[u8]> for FirmwareVersion {
    fn from(val: &[u8]) -> Self {
        let version = std::str::from_utf8(val)
            .unwrap_or("0000")
            // trim leading zeroes
            .trim_start_matches(['0', '\0'])
            .parse::<u32>()
            .unwrap_or(0);

        Self(version)
    }
}

impl<const N: usize> From<[u8; N]> for FirmwareVersion {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for FirmwareVersion {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.02}", self.0 as f32 / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::format;

    #[test]
    fn test_firmware_version() {
        let version_bytes = [0x00, 0x33, 0x33, 0x33];
        let exp_version = 333;
        let exp_version_str = "3.33";

        let version = FirmwareVersion::from(version_bytes);

        assert_eq!(FirmwareVersion::from_inner(exp_version), version);
        assert_eq!(FirmwareVersion::from(b"0333"), version);

        assert_eq!(version.as_inner(), exp_version);

        assert_eq!(format!("{version}").as_str(), exp_version_str);

        assert_eq!(
            FirmwareVersion::from([0x00, 0x00, 0x00, 0x00]),
            FirmwareVersion::from(0)
        );
    }
}
