use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::REJECT_RESPONSE, message::MessageOps, MessageType,
};

/// Reject - Response (0x08)
///
/// Represents a response to an [RejectCommand](crate::RejectCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RejectResponse {
    buf: [u8; REJECT_RESPONSE],
}

impl RejectResponse {
    /// Creates a new [RejectResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; REJECT_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(RejectResponse);
impl_message_from_buf!(RejectResponse);
impl_message_ops!(RejectResponse, MessageType::Reject);
impl_response_ops!(RejectResponse);
impl_response_display!(RejectResponse);
