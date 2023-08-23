use crate::{len::DISABLE_PAYOUT_RESPONSE, message::MessageOps, MessageType};

/// DisablePayout - Response (0x09)
///
/// Represents a response to an [DisablePayoutCommand](crate::DisablePayoutCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DisablePayoutResponse {
    buf: [u8; DISABLE_PAYOUT_RESPONSE],
}

impl DisablePayoutResponse {
    /// Creates a new [DisablePayoutResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISABLE_PAYOUT_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_message_from_buf!(DisablePayoutResponse);
impl_message_ops!(DisablePayoutResponse, MessageType::DisablePayout);
impl_response_ops!(DisablePayoutResponse);
impl_response_display!(DisablePayoutResponse);
