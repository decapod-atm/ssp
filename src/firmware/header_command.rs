use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::FIRMWARE_HEADER_COMMAND, CommandOps, FirmwareHeader, MessageOps, Result,
};

mod index {
    pub const FIRMWARE_HEADER: usize = 3;
    pub const FIRMWARE_HEADER_END: usize = 131;
}

/// ProgramFirmware - Command (0x0B)
///
/// This two byte command prepares the unit for firmware programming.
///
/// The `FirmwareCode` field defines the type of programming.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareHeaderCommand {
    buf: [u8; FIRMWARE_HEADER_COMMAND],
}

impl FirmwareHeaderCommand {
    /// Creates a new [FirmwareHeaderCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FIRMWARE_HEADER_COMMAND],
        };

        msg.init();
        msg.set_firmware_header(&FirmwareHeader::new()).ok();

        msg
    }

    /// Creates a new [FirmwareHeaderCommand] message.
    pub fn create(header: &FirmwareHeader) -> Result<Self> {
        let mut msg = Self {
            buf: [0u8; FIRMWARE_HEADER_COMMAND],
        };

        msg.init();
        msg.set_firmware_header(header)?;

        Ok(msg)
    }

    /// Gets the [FirmwareCode] for the type of programming the unit expects.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let fw_cmd = ssp::FirmwareHeaderCommand::new();
    /// assert_eq!(fw_cmd.firmware_header().unwrap(), ssp::FirmwareHeader::new());
    /// ```
    pub fn firmware_header(&self) -> Result<FirmwareHeader> {
        FirmwareHeader::try_from(&self.buf[index::FIRMWARE_HEADER..index::FIRMWARE_HEADER_END])
    }

    /// Sets the [FirmwareHeader] for the type of programming the unit expects.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let mut fw_cmd = ssp::FirmwareHeaderCommand::new();
    /// let header = ssp::FirmwareHeader::new();
    /// fw_cmd.set_firmware_header(&header);
    /// assert_eq!(fw_cmd.firmware_header().unwrap(), header);
    /// ```
    pub fn set_firmware_header(&mut self, header: &FirmwareHeader) -> Result<()> {
        header.to_bytes(&mut self.buf[index::FIRMWARE_HEADER..index::FIRMWARE_HEADER_END])
    }
}

impl_default!(FirmwareHeaderCommand);
impl_command_display!(FirmwareHeaderCommand);
impl_message_from_buf!(FirmwareHeaderCommand);
impl_message_ops!(FirmwareHeaderCommand);
impl_command_ops!(FirmwareHeaderCommand);
