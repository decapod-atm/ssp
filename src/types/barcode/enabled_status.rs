use crate::std::fmt;

/// Enabled status of any barcode readers present on the device.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarcodeEnabledStatus {
    None = 0x00,
    Top = 0x01,
    Bottom = 0x02,
    Both = 0x03,
}

impl From<u8> for BarcodeEnabledStatus {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Top,
            2 => Self::Bottom,
            3 => Self::Both,
            _ => Self::None,
        }
    }
}

impl From<BarcodeEnabledStatus> for u8 {
    fn from(val: BarcodeEnabledStatus) -> Self {
        val as u8
    }
}

impl From<&BarcodeEnabledStatus> for u8 {
    fn from(val: &BarcodeEnabledStatus) -> Self {
        (*val).into()
    }
}

impl From<BarcodeEnabledStatus> for &'static str {
    fn from(val: BarcodeEnabledStatus) -> Self {
        match val {
            BarcodeEnabledStatus::None => "None",
            BarcodeEnabledStatus::Top => "Top",
            BarcodeEnabledStatus::Bottom => "Bottom",
            BarcodeEnabledStatus::Both => "Both",
        }
    }
}

impl From<&BarcodeEnabledStatus> for &'static str {
    fn from(val: &BarcodeEnabledStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for BarcodeEnabledStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
