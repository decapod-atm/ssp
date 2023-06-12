use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_response_ops,
    impl_wrapped_message_ops, len, CommandOps, MessageOps, MessageType,
};

/// Wrapped Encrypted Message (0x7E)
///
/// Variable length message that wraps an encrypted SSP (eSSP) message.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct WrappedEncryptedMessage {
    buf: [u8; len::WRAPPED_ENCRYPTED_MESSAGE],
}

impl WrappedEncryptedMessage {
    /// Creates a new [WrappedEncryptedMessage] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::WRAPPED_ENCRYPTED_MESSAGE],
        };

        msg.init();
        msg.set_command(MessageType::Encrypted);

        msg
    }

    /// Gets the wrapped encrypted data.
    pub fn encrypted_data(&self) -> &[u8] {
        use crate::message::index;

        let start = index::DATA + 1;
        let end = start + self.data_len() - 1;

        self.buf[start..end].as_ref()
    }
}

impl_default!(WrappedEncryptedMessage);
impl_command_display!(WrappedEncryptedMessage);
impl_message_from_buf!(WrappedEncryptedMessage);
impl_wrapped_message_ops!(WrappedEncryptedMessage);
impl_command_ops!(WrappedEncryptedMessage);
impl_response_ops!(WrappedEncryptedMessage);
