use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// SetEncryptionKey - Response (0x4B)
///
/// Represents a response to an [SetEncryptionKeyCommand](crate::SetEncryptionKeyCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetEncryptionKeyResponse {
    buf: [u8; len::SET_MODULUS_RESPONSE],
}

impl SetEncryptionKeyResponse {
    /// Creates a new [SetEncryptionKeyResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_MODULUS_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SetEncryptionKeyResponse);
impl_message_from_buf!(SetEncryptionKeyResponse);
impl_message_ops!(SetEncryptionKeyResponse, MessageType::SetEncryptionKey);
impl_response_ops!(SetEncryptionKeyResponse);
impl_response_display!(SetEncryptionKeyResponse);
