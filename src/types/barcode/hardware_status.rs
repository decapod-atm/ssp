use crate::std::fmt;

/// Status of any barcode readers present on the device.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarcodeHardwareStatus {
    None = 0x00,
    TopReader = 0x01,
    BottomReader = 0x02,
    Both = 0x03,
}

impl From<u8> for BarcodeHardwareStatus {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::TopReader,
            2 => Self::BottomReader,
            3 => Self::Both,
            _ => Self::None,
        }
    }
}

impl From<BarcodeHardwareStatus> for u8 {
    fn from(val: BarcodeHardwareStatus) -> Self {
        val as u8
    }
}

impl From<&BarcodeHardwareStatus> for u8 {
    fn from(val: &BarcodeHardwareStatus) -> Self {
        (*val).into()
    }
}

impl From<BarcodeHardwareStatus> for &'static str {
    fn from(val: BarcodeHardwareStatus) -> Self {
        match val {
            BarcodeHardwareStatus::None => "None",
            BarcodeHardwareStatus::TopReader => "Top reader",
            BarcodeHardwareStatus::BottomReader => "Bottom reader",
            BarcodeHardwareStatus::Both => "Both",
        }
    }
}

impl From<&BarcodeHardwareStatus> for &'static str {
    fn from(val: &BarcodeHardwareStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for BarcodeHardwareStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
