use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// EventAck - Response (0x57)
///
/// Represents a response to an [EventAckCommand](crate::EventAckCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventAckResponse {
    buf: [u8; len::EVENT_ACK_RESPONSE],
}

impl EventAckResponse {
    /// Creates a new [EventAckResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::EVENT_ACK_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(EventAckResponse);
impl_message_from_buf!(EventAckResponse);
impl_message_ops!(EventAckResponse, MessageType::EventAck);
impl_response_display!(EventAckResponse);
impl_response_ops!(EventAckResponse);
