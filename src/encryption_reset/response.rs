use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// EncryptionReset - Response (0x61)
///
/// Represents a response to an [EncryptionResetCommand](crate::EncryptionResetCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncryptionResetResponse {
    buf: [u8; len::ENCRYPTION_RESET_RESPONSE],
}

impl EncryptionResetResponse {
    /// Creates a new [EncryptionResetResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENCRYPTION_RESET_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(EncryptionResetResponse);
impl_message_from_buf!(EncryptionResetResponse);
impl_message_ops!(EncryptionResetResponse, MessageType::EncryptionReset);
impl_response_ops!(EncryptionResetResponse);
impl_response_display!(EncryptionResetResponse);
