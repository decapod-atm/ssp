use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::DISPLAY_OFF_COMMAND, CommandOps, MessageOps, MessageType,
};

/// Display Off - Command (0x04)
///
/// Single byte command turns off the bezel light when the unit is enabled.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayOffCommand {
    buf: [u8; DISPLAY_OFF_COMMAND],
}

impl DisplayOffCommand {
    /// Creates a new [DisplayOffCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISPLAY_OFF_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::DisplayOff);

        msg
    }
}

impl_default!(DisplayOffCommand);
impl_command_display!(DisplayOffCommand);
impl_message_from_buf!(DisplayOffCommand);
impl_message_ops!(DisplayOffCommand);
impl_command_ops!(DisplayOffCommand);
