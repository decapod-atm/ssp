use crate::{
    impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops, len, std::fmt,
    BarcodeCharacters, BarcodeConfiguration, BarcodeEnabledStatus, BarcodeFormat, CommandOps,
    MessageOps, MessageType,
};

pub mod index {
    pub const ENABLED_STATUS: usize = 4;
    pub const FORMAT: usize = 5;
    pub const CHARACTERS: usize = 6;
}

/// SetBarcodeReaderConfiguration - Command (0x23)
///
/// Four byte command sets up the validator’s bar code reader configuration.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetBarcodeReaderConfigurationCommand {
    buf: [u8; len::SET_BARCODE_READER_CONFIGURATION_COMMAND],
}

impl SetBarcodeReaderConfigurationCommand {
    /// Creates a new [SetBarcodeReaderConfigurationCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_BARCODE_READER_CONFIGURATION_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetBarcodeReaderConfiguration);

        msg
    }

    /// Gets the [BarcodeConfiguration].
    pub fn configuration(&self) -> BarcodeConfiguration {
        self.buf[index::ENABLED_STATUS..=index::CHARACTERS]
            .as_ref()
            .into()
    }

    /// Sets the [BarcodeConfiguration].
    pub fn set_configuration(&mut self, config: BarcodeConfiguration) {
        self.buf[index::ENABLED_STATUS..=index::CHARACTERS]
            // skip the BarcodeHardwareStatus field, since it cannot be set by software
            .copy_from_slice(config.as_bytes()[1..].as_ref())
    }

    /// Gets the [BarcodeEnabledStatus].
    pub fn enabled_status(&self) -> BarcodeEnabledStatus {
        self.buf[index::ENABLED_STATUS].into()
    }

    /// Sets the [BarcodeEnabledStatus].
    pub fn set_enabled_status(&mut self, status: BarcodeEnabledStatus) {
        self.buf[index::ENABLED_STATUS] = status.into();
    }

    /// Gets the [BarcodeFormat].
    pub fn format(&self) -> BarcodeFormat {
        self.buf[index::FORMAT].into()
    }

    /// Sets the [BarcodeFormat].
    pub fn set_format(&mut self, format: BarcodeFormat) {
        self.buf[index::FORMAT] = format.into();
    }

    /// Gets the number of [BarcodeCharacters].
    pub fn num_characters(&self) -> BarcodeCharacters {
        self.buf[index::CHARACTERS].into()
    }

    /// Sets the number of [BarcodeCharacters].
    pub fn set_num_characters(&mut self, num: BarcodeCharacters) {
        self.buf[index::CHARACTERS] = num.into();
    }
}

impl_default!(SetBarcodeReaderConfigurationCommand);
impl_message_from_buf!(SetBarcodeReaderConfigurationCommand);
impl_message_ops!(SetBarcodeReaderConfigurationCommand);
impl_command_ops!(SetBarcodeReaderConfigurationCommand);

impl fmt::Display for SetBarcodeReaderConfigurationCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let command = self.command();
        let config = self.configuration();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Command: {command} | {config} | CRC-16: 0x{crc:04x}")
    }
}
