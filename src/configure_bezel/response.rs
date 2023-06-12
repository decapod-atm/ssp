use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// ConfigureBezel - Response (0x54)
///
/// Represents a response to an [ConfigureBezelCommand](crate::ConfigureBezelCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfigureBezelResponse {
    buf: [u8; len::CONFIGURE_BEZEL_RESPONSE],
}

impl ConfigureBezelResponse {
    /// Creates a new [ConfigureBezelResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::CONFIGURE_BEZEL_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(ConfigureBezelResponse);
impl_message_from_buf!(ConfigureBezelResponse);
impl_message_ops!(ConfigureBezelResponse, MessageType::ConfigureBezel);
impl_response_ops!(ConfigureBezelResponse);
impl_response_display!(ConfigureBezelResponse);
