use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, FixedKey, MessageOps, MessageType,
};

mod index {
    pub const FIXED_KEY: usize = 4;
    pub const FIXED_KEY_END: usize = 12;
}

/// SetEncryptionKey - Command (0x4B)
///
/// Eight data bytes are sent. This is a 64 bit number representing the EncryptionKey, and must be a
/// prime number.
///
/// The device will reply with OK or PARAMETER_OUT_OF_RANGE if the number is not prime.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetEncryptionKeyCommand {
    buf: [u8; len::SET_ENCRYPTION_KEY_COMMAND],
}

impl SetEncryptionKeyCommand {
    /// Creates a new [SetEncryptionKeyCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_ENCRYPTION_KEY_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetEncryptionKey);

        msg
    }

    /// Gets the [FixedKey].
    pub fn fixed_key(&self) -> FixedKey {
        let key_bytes: [u8; 8] = self.buf[index::FIXED_KEY..index::FIXED_KEY_END]
            .try_into()
            .unwrap();
        u64::from_le_bytes(key_bytes).into()
    }

    /// Sets the [FixedKey].
    pub fn set_fixed_key(&mut self, key: &FixedKey) {
        let key_bytes = key.as_inner().to_le_bytes();
        self.buf[index::FIXED_KEY..index::FIXED_KEY_END].copy_from_slice(key_bytes.as_ref());
    }
}

impl_default!(SetEncryptionKeyCommand);
impl_command_display!(SetEncryptionKeyCommand);
impl_message_from_buf!(SetEncryptionKeyCommand);
impl_message_ops!(SetEncryptionKeyCommand);
impl_command_ops!(SetEncryptionKeyCommand);
