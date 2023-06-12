use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Poll with ACK - Command (0x56)
///
/// Single byte command causes the validator to respond to a poll in the same way as normal
/// but specified events will need to be acknowledged by the host using the
/// [EventAckCommand](crate::EventAckCommand) before the validator will allow any further note action.
///
/// If this command is not supported, `0xF2 (Unknown command)` will be returned.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PollWithAckCommand {
    buf: [u8; len::POLL_WITH_ACK_COMMAND],
}

impl PollWithAckCommand {
    /// Creates a new [PollWithAckCommand].
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::POLL_WITH_ACK_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::PollWithAck);

        msg
    }
}

impl_default!(PollWithAckCommand);
impl_command_display!(PollWithAckCommand);
impl_message_from_buf!(PollWithAckCommand);
impl_message_ops!(PollWithAckCommand);
impl_command_ops!(PollWithAckCommand);
