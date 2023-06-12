use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::SERIAL_NUMBER_COMMAND, CommandOps, MessageOps, MessageType,
};

/// SerialNumber - Command (0x0A)
///
/// Single byte command enables the unit. It will now respond to and execute commands.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SerialNumberCommand {
    buf: [u8; SERIAL_NUMBER_COMMAND],
}

impl SerialNumberCommand {
    /// Creates a new [SerialNumberCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; SERIAL_NUMBER_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SerialNumber);

        msg
    }
}

impl_default!(SerialNumberCommand);
impl_command_display!(SerialNumberCommand);
impl_message_from_buf!(SerialNumberCommand);
impl_message_ops!(SerialNumberCommand);
impl_command_ops!(SerialNumberCommand);
