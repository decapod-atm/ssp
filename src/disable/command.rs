use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::DISABLE_COMMAND, CommandOps, MessageOps, MessageType,
};

/// Disable - Command (0x09)
///
/// This single byte command disables the unit. This means the unit will enter its disabled state
/// and not execute any further commands or perform any other actions. A poll to the unit
/// while in this state will report disabled (0xE8)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisableCommand {
    buf: [u8; DISABLE_COMMAND],
}

impl DisableCommand {
    /// Creates a new [DisableCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISABLE_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Disable);

        msg
    }
}

impl_default!(DisableCommand);
impl_command_display!(DisableCommand);
impl_message_from_buf!(DisableCommand);
impl_message_ops!(DisableCommand);
impl_command_ops!(DisableCommand);
