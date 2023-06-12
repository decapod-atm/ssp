use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::UNIT_DATA_COMMAND, CommandOps, MessageOps, MessageType,
};

/// UnitData - Command (0x0D)
///
/// Single byte command causes the validator to return information about itself. It is similar to
/// the Setup Request command but a more concise version. It is intended for host machines
/// with limited resources.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitDataCommand {
    buf: [u8; UNIT_DATA_COMMAND],
}

impl UnitDataCommand {
    /// Creates a new [UnitDataCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; UNIT_DATA_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::UnitData);

        msg
    }
}

impl_default!(UnitDataCommand);
impl_command_display!(UnitDataCommand);
impl_message_from_buf!(UnitDataCommand);
impl_message_ops!(UnitDataCommand);
impl_command_ops!(UnitDataCommand);
