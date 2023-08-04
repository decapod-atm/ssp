use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len, std::fmt,
    BarcodeCharacters, BarcodeConfiguration, BarcodeEnabledStatus, BarcodeFormat,
    BarcodeHardwareStatus, MessageOps, MessageType, ResponseOps,
};

mod index {
    pub const HARDWARE_STATUS: usize = 4;
    pub const ENABLED_STATUS: usize = 5;
    pub const FORMAT: usize = 6;
    pub const CHARACTERS: usize = 7;
}

/// GetBarcodeReaderConfiguration - Response (0x24)
///
/// Represents a response to an [GetBarcodeReaderConfigurationCommand](crate::GetBarcodeReaderConfigurationCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetBarcodeReaderConfigurationResponse {
    buf: [u8; len::GET_BARCODE_READER_CONFIGURATION_RESPONSE],
}

impl GetBarcodeReaderConfigurationResponse {
    /// Creates a new [GetBarcodeReaderConfigurationResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::GET_BARCODE_READER_CONFIGURATION_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [BarcodeConfiguration].
    pub fn configuration(&self) -> BarcodeConfiguration {
        if self.response_status().is_ok() {
            self.buf[index::HARDWARE_STATUS..=index::CHARACTERS]
                .as_ref()
                .into()
        } else {
            BarcodeConfiguration::new()
        }
    }

    /// Gets the [BarcodeHardwareStatus].
    pub fn hardware_status(&self) -> BarcodeHardwareStatus {
        if self.response_status().is_ok() {
            self.buf[index::HARDWARE_STATUS].into()
        } else {
            BarcodeHardwareStatus::None
        }
    }

    /// Gets the [BarcodeEnabledStatus].
    pub fn enabled_status(&self) -> BarcodeEnabledStatus {
        if self.response_status().is_ok() {
            self.buf[index::ENABLED_STATUS].into()
        } else {
            BarcodeEnabledStatus::None
        }
    }

    /// Gets the [BarcodeFormat].
    pub fn format(&self) -> BarcodeFormat {
        if self.response_status().is_ok() {
            self.buf[index::FORMAT].into()
        } else {
            BarcodeFormat::None
        }
    }

    /// Gets the number of [BarcodeCharacters].
    pub fn num_characters(&self) -> BarcodeCharacters {
        if self.response_status().is_ok() {
            self.buf[index::CHARACTERS].into()
        } else {
            BarcodeCharacters::from(0)
        }
    }
}

impl_default!(GetBarcodeReaderConfigurationResponse);
impl_message_from_buf!(GetBarcodeReaderConfigurationResponse);
impl_var_message_ops!(
    GetBarcodeReaderConfigurationResponse,
    MessageType::GetBarcodeReaderConfiguration
);
impl_response_ops!(GetBarcodeReaderConfigurationResponse);

impl fmt::Display for GetBarcodeReaderConfigurationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let config = self.configuration();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | {config} | CRC-16: 0x{crc:04x}")
    }
}
