use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// SmartEmpty - Response (0x3F)
///
/// Represents a response to an [SmartEmptyCommand](crate::SmartEmptyCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmartEmptyResponse {
    buf: [u8; len::SMART_EMPTY_RESPONSE],
}

impl SmartEmptyResponse {
    /// Creates a new [SmartEmptyResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SMART_EMPTY_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SmartEmptyResponse);
impl_message_from_buf!(SmartEmptyResponse);
impl_message_ops!(SmartEmptyResponse, MessageType::SmartEmpty);
impl_response_ops!(SmartEmptyResponse);
impl_response_display!(SmartEmptyResponse);
