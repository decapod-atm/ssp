use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// Enable - Response (0x0A)
///
/// Represents a response to an [EnableCommand](crate::EnableCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnableResponse {
    buf: [u8; len::ENABLE_RESPONSE],
}

impl EnableResponse {
    /// Creates a new [EnableResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENABLE_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(EnableResponse);
impl_message_from_buf!(EnableResponse);
impl_message_ops!(EnableResponse, MessageType::Enable);
impl_response_ops!(EnableResponse);
impl_response_display!(EnableResponse);
