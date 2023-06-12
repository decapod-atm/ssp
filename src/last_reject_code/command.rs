use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::LAST_REJECT_CODE_COMMAND, CommandOps, MessageOps, MessageType,
};

/// LastRejectCode - Command (0x17)
///
/// Single byte command causes the validator to report the reason for the last note being
/// rejected.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LastRejectCodeCommand {
    buf: [u8; LAST_REJECT_CODE_COMMAND],
}

impl LastRejectCodeCommand {
    /// Creates a new [LastRejectCodeCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; LAST_REJECT_CODE_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::LastRejectCode);

        msg
    }
}

impl_default!(LastRejectCodeCommand);
impl_command_display!(LastRejectCodeCommand);
impl_message_from_buf!(LastRejectCodeCommand);
impl_message_ops!(LastRejectCodeCommand);
impl_command_ops!(LastRejectCodeCommand);
