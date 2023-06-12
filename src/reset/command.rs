use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::RESET_COMMAND, CommandOps, MessageOps, MessageType,
};

/// Reset - Command (0x01)
///
/// Single byte command enables the unit. It will now respond to and execute commands.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResetCommand {
    buf: [u8; RESET_COMMAND],
}

impl ResetCommand {
    /// Creates a new [ResetCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; RESET_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Reset);

        msg
    }
}

impl_default!(ResetCommand);
impl_command_display!(ResetCommand);
impl_message_from_buf!(ResetCommand);
impl_message_ops!(ResetCommand);
impl_command_ops!(ResetCommand);
