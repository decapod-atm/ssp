use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::DISPLAY_ON_RESPONSE, message::MessageOps, MessageType,
};

/// DisplayOn - Response (0x03)
///
/// Represents a response to an [DisplayOnCommand](crate::DisplayOnCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayOnResponse {
    buf: [u8; DISPLAY_ON_RESPONSE],
}

impl DisplayOnResponse {
    /// Creates a new [DisplayOnResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISPLAY_ON_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(DisplayOnResponse);
impl_message_from_buf!(DisplayOnResponse);
impl_message_ops!(DisplayOnResponse, MessageType::DisplayOn);
impl_response_ops!(DisplayOnResponse);
impl_response_display!(DisplayOnResponse);
