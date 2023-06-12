use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Hold - Command (0x18)
///
/// Single byte command causes the validator to hold the current accepted note if the
/// developer does not wish to accept or reject the note with the next command. This also
/// resets the 5 second escrow timer. (Normally after 5 seconds a note is automatically
/// rejected).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoldCommand {
    buf: [u8; len::HOLD_COMMAND],
}

impl HoldCommand {
    /// Creates a new [HoldCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::HOLD_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Hold);

        msg
    }
}

impl_default!(HoldCommand);
impl_command_display!(HoldCommand);
impl_message_from_buf!(HoldCommand);
impl_message_ops!(HoldCommand);
impl_command_ops!(HoldCommand);
