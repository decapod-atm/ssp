use crate::{len, MessageOps, MessageType};

/// EnablePayout - Response (0x5C)
///
/// Represents a response to an [EnablePayoutCommand](crate::EnablePayoutCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnablePayoutResponse {
    buf: [u8; len::ENABLE_PAYOUT_RESPONSE],
}

impl EnablePayoutResponse {
    /// Creates a new [EnablePayoutResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENABLE_PAYOUT_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(EnablePayoutResponse);
impl_message_from_buf!(EnablePayoutResponse);
impl_message_ops!(EnablePayoutResponse, MessageType::EnablePayout);
impl_response_ops!(EnablePayoutResponse);
impl_response_display!(EnablePayoutResponse);
