use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// SetGenerator - Response (0x4A)
///
/// Represents a response to an [SetGeneratorCommand](crate::SetGeneratorCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetGeneratorResponse {
    buf: [u8; len::SET_GENERATOR_RESPONSE],
}

impl SetGeneratorResponse {
    /// Creates a new [SetGeneratorResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_GENERATOR_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SetGeneratorResponse);
impl_message_from_buf!(SetGeneratorResponse);
impl_message_ops!(SetGeneratorResponse, MessageType::SetGenerator);
impl_response_ops!(SetGeneratorResponse);
impl_response_display!(SetGeneratorResponse);
