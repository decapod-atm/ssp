use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Poll - Command (0x7F)
///
/// Single byte command instructs the unit to report all the events that have occurred since the
/// last time a poll was sent to the unit.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PollCommand {
    buf: [u8; len::POLL_COMMAND],
}

impl PollCommand {
    /// Creates a new [PollCommand].
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::POLL_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Poll);

        msg
    }
}

impl_default!(PollCommand);
impl_command_display!(PollCommand);
impl_message_from_buf!(PollCommand);
impl_message_ops!(PollCommand);
impl_command_ops!(PollCommand);
