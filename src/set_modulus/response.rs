use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// SetModulus - Response (0x4B)
///
/// Represents a response to an [SetModulusCommand](crate::SetModulusCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetModulusResponse {
    buf: [u8; len::SET_MODULUS_RESPONSE],
}

impl SetModulusResponse {
    /// Creates a new [SetModulusResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_MODULUS_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SetModulusResponse);
impl_message_from_buf!(SetModulusResponse);
impl_message_ops!(SetModulusResponse, MessageType::SetModulus);
impl_response_ops!(SetModulusResponse);
impl_response_display!(SetModulusResponse);
