use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// EncryptionReset - Command (0x61)
///
/// Resets the fixed encryption key to the device default. The device may have extra security
/// requirements before it will accept this command (e.g. The Hopper must be empty) if these
/// requirements are not met, the device will reply with Command Cannot be Processed. If
/// successful, the device will reply OK, then reset. When it starts up the fixed key will be the
/// default.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncryptionResetCommand {
    buf: [u8; len::ENCRYPTION_RESET_COMMAND],
}

impl EncryptionResetCommand {
    /// Creates a new [EncryptionResetCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENCRYPTION_RESET_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::EncryptionReset);

        msg
    }
}

impl_default!(EncryptionResetCommand);
impl_command_display!(EncryptionResetCommand);
impl_message_from_buf!(EncryptionResetCommand);
impl_message_ops!(EncryptionResetCommand);
impl_command_ops!(EncryptionResetCommand);
