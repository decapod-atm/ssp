use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// Empty - Response (0x3F)
///
/// Represents a response to an [EmptyCommand](crate::EmptyCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmptyResponse {
    buf: [u8; len::EMPTY_RESPONSE],
}

impl EmptyResponse {
    /// Creates a new [EmptyResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::EMPTY_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(EmptyResponse);
impl_message_from_buf!(EmptyResponse);
impl_message_ops!(EmptyResponse, MessageType::Empty);
impl_response_ops!(EmptyResponse);
impl_response_display!(EmptyResponse);
