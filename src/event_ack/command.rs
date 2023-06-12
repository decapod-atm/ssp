use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Event ACK - Command (0x57)
///
/// Single byte command causes validator to continue with operations after it has been
/// sending a repeating Poll ACK response. See GA973 appendix for further details about this
/// command.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventAckCommand {
    buf: [u8; len::EVENT_ACK_COMMAND],
}

impl EventAckCommand {
    /// Creates a new [EventAckCommand].
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::EVENT_ACK_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::EventAck);

        msg
    }
}

impl_default!(EventAckCommand);
impl_command_display!(EventAckCommand);
impl_message_from_buf!(EventAckCommand);
impl_message_ops!(EventAckCommand);
impl_command_ops!(EventAckCommand);
