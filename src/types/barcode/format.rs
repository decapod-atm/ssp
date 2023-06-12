use crate::std::fmt;

/// Barcode formatting style.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarcodeFormat {
    None = 0x00,
    Interleaved2of5 = 0x01,
}

impl From<u8> for BarcodeFormat {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Interleaved2of5,
            _ => Self::None,
        }
    }
}

impl From<BarcodeFormat> for u8 {
    fn from(val: BarcodeFormat) -> Self {
        val as u8
    }
}

impl From<&BarcodeFormat> for u8 {
    fn from(val: &BarcodeFormat) -> Self {
        (*val).into()
    }
}

impl From<BarcodeFormat> for &'static str {
    fn from(val: BarcodeFormat) -> Self {
        match val {
            BarcodeFormat::None => "None",
            BarcodeFormat::Interleaved2of5 => "Interleaved 2 of 5",
        }
    }
}

impl From<&BarcodeFormat> for &'static str {
    fn from(val: &BarcodeFormat) -> Self {
        (*val).into()
    }
}

impl fmt::Display for BarcodeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
