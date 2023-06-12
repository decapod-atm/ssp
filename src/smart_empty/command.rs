use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// Empty - Command (0x52)
///
/// Single byte command that causes the validator to empty all its stored notes to the cashbox
/// and also keep a count of the value emptied. This information can be retrieved using the
/// cashbox payout operation data command once the payout is empty.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmartEmptyCommand {
    buf: [u8; len::SMART_EMPTY_COMMAND],
}

impl SmartEmptyCommand {
    /// Creates a new [SmartEmptyCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SMART_EMPTY_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SmartEmpty);

        msg
    }
}

impl_default!(SmartEmptyCommand);
impl_command_display!(SmartEmptyCommand);
impl_message_from_buf!(SmartEmptyCommand);
impl_message_ops!(SmartEmptyCommand);
impl_command_ops!(SmartEmptyCommand);
