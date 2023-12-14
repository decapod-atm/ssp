use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::PROGRAM_FIRMWARE_COMMAND, CommandOps, MessageOps, MessageType,
};

mod index {
    pub const FIRMWARE_CODE: usize = 4;
}

/// Represents the type of programming the unit expects.
// FIXME: there is currently only one code described in public documentation.
// Are there others still in use?
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramFirmwareCode {
    Ram = 0x03,
    Reserved(u8),
}

impl From<u8> for ProgramFirmwareCode {
    fn from(val: u8) -> Self {
        match val {
            0x03 => Self::Ram,
            _ => Self::Reserved(val),
        }
    }
}

impl From<&ProgramFirmwareCode> for u8 {
    fn from(val: &ProgramFirmwareCode) -> Self {
        match val {
            ProgramFirmwareCode::Ram => 0x03,
            ProgramFirmwareCode::Reserved(code) => *code,
        }
    }
}

impl From<ProgramFirmwareCode> for u8 {
    fn from(val: ProgramFirmwareCode) -> Self {
        (&val).into()
    }
}

/// ProgramFirmware - Command (0x0B)
///
/// This two byte command prepares the unit for firmware programming.
///
/// The `FirmwareCode` field defines the type of programming.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgramFirmwareCommand {
    buf: [u8; PROGRAM_FIRMWARE_COMMAND],
}

impl ProgramFirmwareCommand {
    /// Creates a new [SyncCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; PROGRAM_FIRMWARE_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::ProgramFirmware);
        msg.set_firmware_code(ProgramFirmwareCode::Ram);

        msg
    }

    /// Gets the [FirmwareCode] for the type of programming the unit expects.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let fw_cmd = ssp::ProgramFirmwareCommand::new();
    /// assert_eq!(fw_cmd.firmware_code(), ssp::ProgramFirmwareCode::Ram);
    /// ```
    pub fn firmware_code(&self) -> ProgramFirmwareCode {
        self.buf[index::FIRMWARE_CODE].into()
    }

    /// Sets the [FirmwareCode] for the type of programming the unit expects.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let mut fw_cmd = ssp::ProgramFirmwareCommand::new();
    /// fw_cmd.set_firmware_code(ssp::ProgramFirmwareCode::Ram);
    /// assert_eq!(fw_cmd.firmware_code(), ssp::ProgramFirmwareCode::Ram);
    /// ```
    pub fn set_firmware_code(&mut self, code: ProgramFirmwareCode) {
        self.buf[index::FIRMWARE_CODE] = code.into();
    }
}

impl_default!(ProgramFirmwareCommand);
impl_command_display!(ProgramFirmwareCommand);
impl_message_from_buf!(ProgramFirmwareCommand);
impl_message_ops!(ProgramFirmwareCommand);
impl_command_ops!(ProgramFirmwareCommand);
