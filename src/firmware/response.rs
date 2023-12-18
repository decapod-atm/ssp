use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::PROGRAM_FIRMWARE_RESPONSE, message::MessageOps, MessageType,
};

mod index {
    pub const BLOCK_LEN: usize = 4;
    pub const BLOCK_LEN_END: usize = 5;
}

/// ProgramFirmware - Response (0x0B)
///
/// Represents a response to an [ProgramFirmwareCommand](crate::ProgramFirmwareCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgramFirmwareResponse {
    buf: [u8; PROGRAM_FIRMWARE_RESPONSE],
}

impl ProgramFirmwareResponse {
    /// Creates a new [ProgramFirmwareResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; PROGRAM_FIRMWARE_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the expected block length from the device.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let fw_res = ssp::ProgramFirmwareResponse::new();
    /// assert_eq!(fw_res.block_len(), 0);
    /// ```
    pub fn block_len(&self) -> u16 {
        u16::from_le_bytes([self.buf[index::BLOCK_LEN], self.buf[index::BLOCK_LEN_END]])
    }

    /// Sets the expected block length from the device.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let mut fw_res = ssp::ProgramFirmwareResponse::new();
    /// fw_res.set_block_len(128);
    /// assert_eq!(fw_res.block_len(), 128);
    /// ```
    pub fn set_block_len(&mut self, len: u16) {
        self.buf[index::BLOCK_LEN..=index::BLOCK_LEN_END].copy_from_slice(&len.to_le_bytes());
    }
}

impl_default!(ProgramFirmwareResponse);
impl_message_from_buf!(ProgramFirmwareResponse);
impl_message_ops!(ProgramFirmwareResponse, MessageType::ProgramFirmware);
impl_response_ops!(ProgramFirmwareResponse);
impl_response_display!(ProgramFirmwareResponse);
