use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// Hold - Response (0x18)
///
/// Represents a response to an [HoldCommand](crate::HoldCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoldResponse {
    buf: [u8; len::HOLD_RESPONSE],
}

impl HoldResponse {
    /// Creates a new [HoldResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::HOLD_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(HoldResponse);
impl_message_from_buf!(HoldResponse);
impl_message_ops!(HoldResponse, MessageType::Hold);
impl_response_ops!(HoldResponse);
impl_response_display!(HoldResponse);
