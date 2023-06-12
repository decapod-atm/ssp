use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::DISPLAY_OFF_RESPONSE, message::MessageOps, MessageType,
};

/// DisplayOff - Response (0x04)
///
/// Represents a response to an [DisplayOffCommand](crate::DisplayOffCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayOffResponse {
    buf: [u8; DISPLAY_OFF_RESPONSE],
}

impl DisplayOffResponse {
    /// Creates a new [DisplayOffResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISPLAY_OFF_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(DisplayOffResponse);
impl_message_from_buf!(DisplayOffResponse);
impl_message_ops!(DisplayOffResponse, MessageType::DisplayOff);
impl_response_ops!(DisplayOffResponse);
impl_response_display!(DisplayOffResponse);
