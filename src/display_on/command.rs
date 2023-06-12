use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::DISPLAY_ON_COMMAND, CommandOps, MessageOps, MessageType,
};

/// Display On - Command (0x03)
///
/// Single byte command turns on the bezel light when the unit is enabled.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayOnCommand {
    buf: [u8; DISPLAY_ON_COMMAND],
}

impl DisplayOnCommand {
    /// Creates a new [DisplayOnCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISPLAY_ON_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::DisplayOn);

        msg
    }
}

impl_default!(DisplayOnCommand);
impl_command_display!(DisplayOnCommand);
impl_message_from_buf!(DisplayOnCommand);
impl_message_ops!(DisplayOnCommand);
impl_command_ops!(DisplayOnCommand);
