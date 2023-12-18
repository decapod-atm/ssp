use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::FIRMWARE_HEADER_RESPONSE, message::MessageOps, MessageType,
};

/// FirmwareHeader - Response (0x0B)
///
/// Represents a response to an [FirmwareHeaderCommand](crate::FirmwareHeaderCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareHeaderResponse {
    buf: [u8; FIRMWARE_HEADER_RESPONSE],
}

impl FirmwareHeaderResponse {
    /// Creates a new [ProgramFirmwareResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; FIRMWARE_HEADER_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(FirmwareHeaderResponse);
impl_message_from_buf!(FirmwareHeaderResponse);
impl_message_ops!(FirmwareHeaderResponse, MessageType::ProgramFirmware);
impl_response_ops!(FirmwareHeaderResponse);
impl_response_display!(FirmwareHeaderResponse);
