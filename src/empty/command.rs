use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Empty - Command (0x3F)
///
/// Single byte command causes the device to empty all its stored notes to the cashbox.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmptyCommand {
    buf: [u8; len::EMPTY_COMMAND],
}

impl EmptyCommand {
    /// Creates a new [EmptyCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::EMPTY_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Empty);

        msg
    }
}

impl_default!(EmptyCommand);
impl_command_display!(EmptyCommand);
impl_message_from_buf!(EmptyCommand);
impl_message_ops!(EmptyCommand);
impl_command_ops!(EmptyCommand);
