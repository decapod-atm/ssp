use crate::{len::DISABLE_RESPONSE, message::MessageOps, MessageType};

/// Disable - Response (0x09)
///
/// Represents a response to an [DisableCommand](crate::DisableCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DisableResponse {
    buf: [u8; DISABLE_RESPONSE],
}

impl DisableResponse {
    /// Creates a new [DisableResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISABLE_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_message_from_buf!(DisableResponse);
impl_message_ops!(DisableResponse, MessageType::Disable);
impl_response_ops!(DisableResponse);
impl_response_display!(DisableResponse);
