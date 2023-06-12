use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::REJECT_COMMAND, CommandOps, MessageOps, MessageType,
};

/// Reject - Command (0x08)
///
/// Single byte command causes the validator to reject the current note.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RejectCommand {
    buf: [u8; REJECT_COMMAND],
}

impl RejectCommand {
    /// Creates a new [RejectCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; REJECT_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Reject);

        msg
    }
}

impl_default!(RejectCommand);
impl_command_display!(RejectCommand);
impl_message_from_buf!(RejectCommand);
impl_message_ops!(RejectCommand);
impl_command_ops!(RejectCommand);
