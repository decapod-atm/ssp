use crate::{impl_default, std::fmt};

use super::*;

/// Container structure for configuring device barcode readers.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BarcodeConfiguration {
    hardware_status: Option<BarcodeHardwareStatus>,
    enabled_status: BarcodeEnabledStatus,
    format: BarcodeFormat,
    chars: BarcodeCharacters,
}

impl BarcodeConfiguration {
    /// Creates a new [BarcodeConfiguration].
    pub const fn new() -> Self {
        Self {
            hardware_status: None,
            enabled_status: BarcodeEnabledStatus::None,
            format: BarcodeFormat::None,
            chars: BarcodeCharacters(0),
        }
    }

    /// Gets the [BarcodeHardwareStatus](crate::BarcodeHardwareStatus).
    pub fn hardware_status(&self) -> BarcodeHardwareStatus {
        if let Some(status) = self.hardware_status {
            status
        } else {
            BarcodeHardwareStatus::None
        }
    }

    /// Sets the [BarcodeHardwareStatus](crate::BarcodeHardwareStatus).
    pub fn set_hardware_status(&mut self, status: BarcodeHardwareStatus) {
        self.hardware_status = Some(status);
    }

    /// Unsets the [BarcodeHardwareStatus](crate::BarcodeHardwareStatus).
    pub fn unset_hardware_status(&mut self) {
        self.hardware_status = None;
    }

    /// Gets the [BarcodeEnabledStatus](crate::BarcodeEnabledStatus).
    pub fn enabled_status(&self) -> BarcodeEnabledStatus {
        self.enabled_status
    }

    /// Sets the [BarcodeEnabledStatus](crate::BarcodeEnabledStatus).
    pub fn set_enabled_status(&mut self, status: BarcodeEnabledStatus) {
        self.enabled_status = status;
    }

    /// Gets the [BarcodeFormat].
    pub fn format(&self) -> BarcodeFormat {
        self.format
    }

    /// Sets the [BarcodeFormat].
    pub fn set_format(&mut self, format: BarcodeFormat) {
        self.format = format;
    }

    /// Gets the number of [BarcodeCharacters].
    pub fn num_characters(&self) -> BarcodeCharacters {
        self.chars
    }

    /// Sets the number of [BarcodeCharacters].
    pub fn set_num_characters(&mut self, chars: BarcodeCharacters) {
        self.chars = chars;
    }

    /// Gets the [BarcodeConfiguration] as a byte array.
    pub fn as_bytes(&self) -> [u8; 4] {
        [
            self.hardware_status().into(),
            self.enabled_status().into(),
            self.format().into(),
            self.num_characters().into(),
        ]
    }
}

impl From<&[u8]> for BarcodeConfiguration {
    fn from(val: &[u8]) -> Self {
        match val.len() {
            0..=2 => Self::new(),
            3 => Self {
                hardware_status: None,
                enabled_status: val[0].into(),
                format: val[1].into(),
                chars: val[2].into(),
            },
            _ => Self {
                hardware_status: Some(val[0].into()),
                enabled_status: val[1].into(),
                format: val[2].into(),
                chars: val[3].into(),
            },
        }
    }
}

impl<const N: usize> From<[u8; N]> for BarcodeConfiguration {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for BarcodeConfiguration {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl From<&BarcodeConfiguration> for [u8; 4] {
    fn from(val: &BarcodeConfiguration) -> Self {
        val.as_bytes()
    }
}

impl fmt::Display for BarcodeConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let en_status = self.enabled_status();
        let format = self.format();
        let chars = self.num_characters();

        if let Some(hd_status) = self.hardware_status {
            write!(f, "Barcode hardware status: {hd_status} | Barcode enabled status: {en_status} | Barcode format: {format} | Number of barcode characters: {chars}")
        } else {
            write!(f, "Barcode enabled status: {en_status} | Barcode format: {format} | Number of barcode characters: {chars}")
        }
    }
}

impl_default!(BarcodeConfiguration);
