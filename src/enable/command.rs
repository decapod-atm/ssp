use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Enable - Command (0x0A)
///
/// Single byte command enables the unit. It will now respond to and execute commands.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnableCommand {
    buf: [u8; len::ENABLE_COMMAND],
}

impl EnableCommand {
    /// Creates a new [EnableCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENABLE_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Enable);

        msg
    }
}

impl_default!(EnableCommand);
impl_command_display!(EnableCommand);
impl_message_from_buf!(EnableCommand);
impl_message_ops!(EnableCommand);
impl_command_ops!(EnableCommand);
