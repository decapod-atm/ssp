use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::PAYOUT_BY_DENOMINATION_RESPONSE, message::MessageOps, MessageType,
};

/// PayoutByDenomination - Response (0x46)
///
/// Represents a response to an [PayoutByDenominationCommand](crate::PayoutByDenominationCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PayoutByDenominationResponse {
    buf: [u8; PAYOUT_BY_DENOMINATION_RESPONSE],
}

impl PayoutByDenominationResponse {
    /// Creates a new [PayoutByDenominationResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; PAYOUT_BY_DENOMINATION_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(PayoutByDenominationResponse);
impl_message_from_buf!(PayoutByDenominationResponse);
impl_message_ops!(
    PayoutByDenominationResponse,
    MessageType::PayoutByDenomination
);
impl_response_ops!(PayoutByDenominationResponse);
impl_response_display!(PayoutByDenominationResponse);
