use crate::{len::DISABLE_PAYOUT_COMMAND, CommandOps, MessageOps, MessageType};

/// DisablePayout - Command (0x5B)
///
/// All accepted notes will be routed to the stacker and payout commands will not be accepted.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DisablePayoutCommand {
    buf: [u8; DISABLE_PAYOUT_COMMAND],
}

impl DisablePayoutCommand {
    /// Creates a new [DisablePayoutCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DISABLE_PAYOUT_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::DisablePayout);

        msg
    }
}

impl_command_display!(DisablePayoutCommand);
impl_message_from_buf!(DisablePayoutCommand);
impl_message_ops!(DisablePayoutCommand);
impl_command_ops!(DisablePayoutCommand);
